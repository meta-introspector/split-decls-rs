// Generated macro for impl_250 (impl)
macro_rules! Depcrate_events_attributesimpl_250 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_250"}
// Dependencies: {}
impl < 'a > From < (& 'a [u8] , & 'a [u8]) > for Attribute < 'a > { # [doc = " Creates new attribute from raw bytes."] # [doc = " Does not apply any transformation to both key and value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use pretty_assertions::assert_eq;"] # [doc = " use quick_xml::events::attributes::Attribute;"] # [doc = ""] # [doc = " let features = Attribute::from((\"features\".as_bytes(), \"Bells &amp; whistles\".as_bytes()));"] # [doc = " assert_eq!(features.value, \"Bells &amp; whistles\".as_bytes());"] # [doc = " ```"] fn from (val : (& 'a [u8] , & 'a [u8])) -> Attribute < 'a > { Attribute { key : QName (val . 0) , value : Cow :: from (val . 1) , } } }
};
}
