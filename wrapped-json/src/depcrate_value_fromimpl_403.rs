// Generated macro for impl_403 (impl)
macro_rules! Depcrate_value_fromimpl_403 {
() => {
// Module: crate::value::from
// Provides: {"impl_403"}
// Dependencies: {}
impl From < String > for Value { # [doc = " Convert `String` to `Value::String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let s: String = \"lorem\".to_owned();"] # [doc = " let x: Value = s.into();"] # [doc = " ```"] fn from (f : String) -> Self { Value :: String (f) } }
};
}
