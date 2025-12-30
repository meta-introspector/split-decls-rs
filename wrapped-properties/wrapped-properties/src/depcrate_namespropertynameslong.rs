// Generated macro for PropertyNamesLong (struct)
macro_rules! Depcrate_namesPropertyNamesLong {
() => {
// Module: crate::names
// Provides: {"PropertyNamesLong"}
// Dependencies: {}
# [doc = " A struct capable of looking up a property name from a value"] # [doc = " Access its data by calling [`Self::as_borrowed()`] and using the methods on"] # [doc = " [`PropertyNamesLongBorrowed`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::props::CanonicalCombiningClass;"] # [doc = " use icu::properties::PropertyNamesLong;"] # [doc = ""] # [doc = " let names = PropertyNamesLong::<CanonicalCombiningClass>::new();"] # [doc = " assert_eq!("] # [doc = "     names.get(CanonicalCombiningClass::KanaVoicing),"] # [doc = "     Some(\"Kana_Voicing\")"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     names.get(CanonicalCombiningClass::AboveLeft),"] # [doc = "     Some(\"Above_Left\")"] # [doc = " );"] # [doc = " ```"] pub struct PropertyNamesLong < T : NamedEnumeratedProperty > { map : DataPayload < ErasedMarker < T :: DataStructLong > > , }
};
}
