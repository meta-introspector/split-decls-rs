macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl From < String > for Value { # [doc = " Convert `String` to `Value::String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let s: String = \"lorem\".to_owned();"] # [doc = " let x: Value = s.into();"] # [doc = " ```"] fn from (f : String) -> Self { Value :: String (f) } }
    };
}

impl_302!();