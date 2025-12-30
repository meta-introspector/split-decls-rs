// Generated macro for impl_81 (impl)
macro_rules! Depcrate_namesimpl_81 {
() => {
// Module: crate::names
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a , T : NamedEnumeratedProperty > PropertyNamesShortBorrowed < 'a , T > { # [doc = " Get the property name given a value"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::properties::props::CanonicalCombiningClass;"] # [doc = " use icu::properties::PropertyNamesShort;"] # [doc = ""] # [doc = " let lookup = PropertyNamesShort::<CanonicalCombiningClass>::new();"] # [doc = " assert_eq!(lookup.get(CanonicalCombiningClass::KanaVoicing), Some(\"KV\"));"] # [doc = " assert_eq!(lookup.get(CanonicalCombiningClass::AboveLeft), Some(\"AL\"));"] # [doc = " ```"] # [inline] pub fn get (self , property : T) -> Option < & 'a str > { self . map . get (property . to_u32 ()) } }
};
}
