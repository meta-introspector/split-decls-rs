// Generated macro for impl_410 (impl)
macro_rules! Depcrate_value_fromimpl_410 {
() => {
// Module: crate::value::from
// Provides: {"impl_410"}
// Dependencies: {}
impl < T : Clone + Into < Value > > From < & [T] > for Value { # [doc = " Convert a slice to `Value::Array`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let v: &[&str] = &[\"lorem\", \"ipsum\", \"dolor\"];"] # [doc = " let x: Value = v.into();"] # [doc = " ```"] fn from (f : & [T]) -> Self { Value :: Array (f . iter () . cloned () . map (Into :: into) . collect ()) } }
};
}
