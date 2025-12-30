// Generated macro for impl_401 (impl)
macro_rules! Depcrate_value_fromimpl_401 {
() => {
// Module: crate::value::from
// Provides: {"impl_401"}
// Dependencies: {}
impl From < f64 > for Value { # [doc = " Convert 64-bit floating point number to `Value::Number`, or"] # [doc = " `Value::Null` if infinite or NaN."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let f: f64 = 13.37;"] # [doc = " let x: Value = f.into();"] # [doc = " ```"] fn from (f : f64) -> Self { Number :: from_f64 (f) . map_or (Value :: Null , Value :: Number) } }
};
}
