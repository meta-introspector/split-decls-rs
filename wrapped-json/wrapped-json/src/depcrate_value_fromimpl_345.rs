// Generated macro for impl_345 (impl)
macro_rules! Depcrate_value_fromimpl_345 {
() => {
// Module: crate::value::from
// Provides: {"impl_345"}
// Dependencies: {}
impl From < Number > for Value { # [doc = " Convert `Number` to `Value::Number`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::{Number, Value};"] # [doc = ""] # [doc = " let n = Number::from(7);"] # [doc = " let x: Value = n.into();"] # [doc = " ```"] fn from (f : Number) -> Self { Value :: Number (f) } }
};
}
