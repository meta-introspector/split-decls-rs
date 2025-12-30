// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl BidiClass { # [doc = " Returns the corresponding `BidiClassMask`."] # [inline (always)] pub fn to_mask (self) -> BidiClassMask { BidiClassMask (bidi_class_to_mask (self . 0)) } # [doc = " `true` iff this value is Left_To_Right"] # [inline (always)] pub fn is_ltr (self) -> bool { self . 0 == icu_properties :: props :: BidiClass :: LeftToRight } # [doc = " `true` iff this value is Nonspacing_Mark"] # [inline (always)] pub fn is_nonspacing_mark (self) -> bool { self . 0 == icu_properties :: props :: BidiClass :: NonspacingMark } # [doc = " `true` iff this value is European_Number"] # [inline (always)] pub fn is_european_number (self) -> bool { self . 0 == icu_properties :: props :: BidiClass :: EuropeanNumber } # [doc = " `true` iff this value is Arabic_Number"] # [inline (always)] pub fn is_arabic_number (self) -> bool { self . 0 == icu_properties :: props :: BidiClass :: ArabicNumber } }
};
}
