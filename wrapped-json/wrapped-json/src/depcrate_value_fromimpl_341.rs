// Generated macro for impl_341 (impl)
macro_rules! Depcrate_value_fromimpl_341 {
() => {
// Module: crate::value::from
// Provides: {"impl_341"}
// Dependencies: {}
impl From < bool > for Value { # [doc = " Convert boolean to `Value::Bool`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let b = false;"] # [doc = " let x: Value = b.into();"] # [doc = " ```"] fn from (f : bool) -> Self { Value :: Bool (f) } }
};
}
