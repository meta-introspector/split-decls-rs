macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl From < bool > for Value { # [doc = " Convert boolean to `Value::Bool`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let b = false;"] # [doc = " let x: Value = b.into();"] # [doc = " ```"] fn from (f : bool) -> Self { Value :: Bool (f) } }
    };
}

impl_301!();