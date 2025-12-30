// Generated macro for impl_72 (impl)
macro_rules! Depcrate_namesimpl_72 {
() => {
// Module: crate::names
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a , T : NamedEnumeratedProperty > PropertyNamesLongBorrowed < 'a , T > { # [doc = " Get the property name given a value"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::properties::props::CanonicalCombiningClass;"] # [doc = " use icu::properties::PropertyNamesLong;"] # [doc = ""] # [doc = " let lookup = PropertyNamesLong::<CanonicalCombiningClass>::new();"] # [doc = " assert_eq!("] # [doc = "     lookup.get(CanonicalCombiningClass::KanaVoicing),"] # [doc = "     Some(\"Kana_Voicing\")"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     lookup.get(CanonicalCombiningClass::AboveLeft),"] # [doc = "     Some(\"Above_Left\")"] # [doc = " );"] # [doc = " ```"] # [inline] pub fn get (self , property : T) -> Option < & 'a str > { self . map . get (property . to_u32 ()) } }
};
}
