// Generated macro for impl_351 (impl)
macro_rules! Depcrate_value_fromimpl_351 {
() => {
// Module: crate::value::from
// Provides: {"impl_351"}
// Dependencies: {}
impl < K : Into < String > , V : Into < Value > > FromIterator < (K , V) > for Value { # [doc = " Create a `Value::Object` by collecting an iterator of key-value pairs."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let v: Vec<_> = vec![(\"lorem\", 40), (\"ipsum\", 2)];"] # [doc = " let x: Value = v.into_iter().collect();"] # [doc = " ```"] fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { Value :: Object (iter . into_iter () . map (| (k , v) | (k . into () , v . into ())) . collect () ,) } }
};
}
