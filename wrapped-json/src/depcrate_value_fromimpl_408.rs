// Generated macro for impl_408 (impl)
macro_rules! Depcrate_value_fromimpl_408 {
() => {
// Module: crate::value::from
// Provides: {"impl_408"}
// Dependencies: {}
impl < T : Into < Value > > From < Vec < T > > for Value { # [doc = " Convert a `Vec` to `Value::Array`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let v = vec![\"lorem\", \"ipsum\", \"dolor\"];"] # [doc = " let x: Value = v.into();"] # [doc = " ```"] fn from (f : Vec < T >) -> Self { Value :: Array (f . into_iter () . map (Into :: into) . collect ()) } }
};
}
