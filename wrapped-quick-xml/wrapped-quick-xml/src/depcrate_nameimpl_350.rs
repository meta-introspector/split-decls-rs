// Generated macro for impl_350 (impl)
macro_rules! Depcrate_nameimpl_350 {
() => {
// Module: crate::name
// Provides: {"impl_350"}
// Dependencies: {}
impl < 'a > From < QName < 'a > > for LocalName < 'a > { # [doc = " Creates `LocalName` from a [`QName`]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use quick_xml::name::{LocalName, QName};"] # [doc = ""] # [doc = " let local: LocalName = QName(b\"unprefixed\").into();"] # [doc = " assert_eq!(local.as_ref(), b\"unprefixed\");"] # [doc = ""] # [doc = " let local: LocalName = QName(b\"some:prefix\").into();"] # [doc = " assert_eq!(local.as_ref(), b\"prefix\");"] # [doc = " ```"] # [inline] fn from (name : QName < 'a >) -> Self { Self (name . index () . map_or (name . 0 , | i | & name . 0 [i + 1 ..])) } }
};
}
