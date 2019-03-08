mod ast;
mod utils;

use utils::TimeUtil;
use ast::NodeString::NodeString;
//use utils::TimeUtil;
use chrono::Local;
use std::collections::HashMap;
use crate::ast::Node::Node;

fn main() {
    let node_string = NodeString { buf: "".to_string() };

    let mut m = HashMap::new();
    m.insert("v",String::from("dsa"));


    let s = node_string.Eval(&m);
    println!("{}", s);


    let v = m["v"].as_str();


    let re = &String::from("s");
    let asf = v;
    println!("{}", asf);


    TimeUtil::Count_Time(1, Local::now());
}