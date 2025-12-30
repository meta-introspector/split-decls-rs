// Generated macro for impl_251 (impl)
macro_rules! Depcrate_events_attributesimpl_251 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_251"}
// Dependencies: {}
impl < 'a > From < (& 'a str , & 'a str) > for Attribute < 'a > { # [doc = " Creates new attribute from text representation."] # [doc = " Key is stored as-is, but the value will be escaped."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " use quick_xml::events::attributes::Attribute;"] # [doc = ""] # [doc = " let features = Attribute::from((\"features\", \"Bells & whistles\"));"] # [doc = " assert_eq!(features.value, \"Bells &amp; whistles\".as_bytes());"] # [doc = " ```"] fn from (val : (& 'a str , & 'a str)) -> Attribute < 'a > { Attribute { key : QName (val . 0 . as_bytes ()) , value : match escape (val . 1) { Cow :: Borrowed (s) => Cow :: Borrowed (s . as_bytes ()) , Cow :: Owned (s) => Cow :: Owned (s . into_bytes ()) , } , } } }
};
}
