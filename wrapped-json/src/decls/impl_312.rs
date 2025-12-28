macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl From < () > for Value { # [doc = " Convert `()` to `Value::Null`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let u = ();"] # [doc = " let x: Value = u.into();"] # [doc = " ```"] fn from (() : ()) -> Self { Value :: Null } }
    };
}

impl_312!();