macro_rules! deps {
    () => {
        Value!();
        Number!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl From < Number > for Value { # [doc = " Convert `Number` to `Value::Number`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::{Number, Value};"] # [doc = ""] # [doc = " let n = Number::from(7);"] # [doc = " let x: Value = n.into();"] # [doc = " ```"] fn from (f : Number) -> Self { Value :: Number (f) } }
    };
}

impl_305!()