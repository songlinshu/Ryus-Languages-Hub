#![allow(unused_variables)]
#![allow(dead_code)]

static KEY_WORD: &'static [&'static str] = &["module", "func", "param", "local", "result",
                                             "export", "call"];

static TYPE: &'static [&'static str] = &["i32", "i64", "f32", "f64"];
static FUNC_FOR_TYPE_CLASS: &'static [&'static str] = &["add"];

static FUNC: &'static [&'static str] = &["get_local"];

#[derive(Debug)]
pub enum Expr {
    Key(String),
    Type(String),
    Ident(String),
    Func(Option<String>, String),
    Integer(i64),
    Float(f64),
    Child(Vec<Expr>),
}

fn is_key(code: String) -> Option<Expr> {
    for i in KEY_WORD.iter() {
        if &*code.trim() == *i {
            return Some(Expr::Key(i.to_string()));
        }
    }
    None
}

fn is_type(code: String) -> Option<Expr> {
    for i in TYPE.iter() {
        if &*code.trim() == *i {
            return Some(Expr::Type(i.to_string()));
        }
    }
    None
}

fn is_ident(code: String) -> Option<Expr> {
    let _code = code.trim();
    if !_code.starts_with("$") {
        return None;
    }
    let mut result = "_".to_string();
    for (index, i) in _code.chars().enumerate() {
        if index == 0 {
            continue;
        }
        match i {
            '0'...'9' => result.push(i),
            'a'...'z' | 'A'...'Z' => result.push(i),
            '_' | '-' => result.push(i),
            '*' | '/' | '\\' | '^' | '~' | '.' | '+' | '=' | '<' | '>' | '!' | '?' | '@' |
            '#' | '$' | '%' | '\'' | '&' | '|' | ':' | '`' => return None,
            _ => return None,
        }
    }
    Some(Expr::Ident(result))
}

fn is_func(code: String) -> Option<Expr> {
    // is global function
    for i in FUNC.iter() {
        if &*code.trim() == *i {
            return Some(Expr::Func(None, i.to_string()));
        }
    }
    // is function for type class
    for class in TYPE.iter() {
        for i in FUNC_FOR_TYPE_CLASS.iter() {
            if code.trim() == class.to_string() + "." + *i {
                return Some(Expr::Func(Some(class.to_string()), i.to_string()));
            }
        }
    }
    None
}

fn is_number(code: String) -> Option<Expr> {
    let _code = code.trim();
    let mut is_float = false;
    let mut is_negative = false;
    let mut result = "".to_string();
    for i in _code.chars() {
        match i {
            '0'...'9' => {}
            '.' => {
                if is_float {
                    return None;
                }
                is_float = true;
            }
            '-' => {
                if is_negative {
                    return None;
                }
                is_negative = true;
            }
            _ => return None,
        }
        result.push(i);
    }
    Some(if is_float {
             Expr::Float(result.parse::<f64>().unwrap())
         } else {
             Expr::Integer(result.parse::<i64>().unwrap())
         })
}

fn get_token(code: String) -> Option<Vec<Expr>> {
    None
}

fn wat2rust(code: String) -> Option<String> {
    None
}

fn main() {
    println!("{:?}", is_key("func".to_string()));
    println!("{:?}", is_type("i32".to_string()));
    println!("{:?}", is_func("get_local".to_string()));
    println!("{:?}", is_func("i32.add".to_string()));
    println!("{:?}", is_ident("$123".to_string()));
    println!("{:?}", is_number("-123".to_string()));
    println!("{:?}", is_number("-12.3".to_string()));
}
