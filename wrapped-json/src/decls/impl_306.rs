macro_rules! deps {
    () => {
        Map!();
        Value!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl From < Map < String , Value > > for Value { # [doc = " Convert map (with string keys) to `Value::Object`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::{Map, Value};"] # [doc = ""] # [doc = " let mut m = Map::new();"] # [doc = " m.insert(\"Lorem\".to_owned(), \"ipsum\".into());"] # [doc = " let x: Value = m.into();"] # [doc = " ```"] fn from (f : Map < String , Value >) -> Self { Value :: Object (f) } }
    };
}

impl_306!();