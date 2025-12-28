macro_rules! deps {
    () => {
        Value!();
        Number!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl From < f32 > for Value { # [doc = " Convert 32-bit floating point number to `Value::Number`, or"] # [doc = " `Value::Null` if infinite or NaN."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let f: f32 = 13.37;"] # [doc = " let x: Value = f.into();"] # [doc = " ```"] fn from (f : f32) -> Self { Number :: from_f32 (f) . map_or (Value :: Null , Value :: Number) } }
    };
}

impl_299!()