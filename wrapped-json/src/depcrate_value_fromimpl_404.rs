// Generated macro for impl_404 (impl)
macro_rules! Depcrate_value_fromimpl_404 {
() => {
// Module: crate::value::from
// Provides: {"impl_404"}
// Dependencies: {}
impl From < & str > for Value { # [doc = " Convert string slice to `Value::String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let s: &str = \"lorem\";"] # [doc = " let x: Value = s.into();"] # [doc = " ```"] fn from (f : & str) -> Self { Value :: String (f . to_owned ()) } }
};
}
