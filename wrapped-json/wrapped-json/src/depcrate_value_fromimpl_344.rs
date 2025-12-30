// Generated macro for impl_344 (impl)
macro_rules! Depcrate_value_fromimpl_344 {
() => {
// Module: crate::value::from
// Provides: {"impl_344"}
// Dependencies: {}
impl < 'a > From < Cow < 'a , str > > for Value { # [doc = " Convert copy-on-write string to `Value::String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = " use std::borrow::Cow;"] # [doc = ""] # [doc = " let s: Cow<str> = Cow::Borrowed(\"lorem\");"] # [doc = " let x: Value = s.into();"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = " use std::borrow::Cow;"] # [doc = ""] # [doc = " let s: Cow<str> = Cow::Owned(\"lorem\".to_owned());"] # [doc = " let x: Value = s.into();"] # [doc = " ```"] fn from (f : Cow < 'a , str >) -> Self { Value :: String (f . into_owned ()) } }
};
}
