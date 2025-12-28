macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl From < & str > for Value { # [doc = " Convert string slice to `Value::String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let s: &str = \"lorem\";"] # [doc = " let x: Value = s.into();"] # [doc = " ```"] fn from (f : & str) -> Self { Value :: String (f . to_owned ()) } }
    };
}

impl_303!();