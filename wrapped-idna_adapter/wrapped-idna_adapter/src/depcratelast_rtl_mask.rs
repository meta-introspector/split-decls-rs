// Generated macro for LAST_RTL_MASK (const)
macro_rules! DepcrateLAST_RTL_MASK {
() => {
// Module: crate
// Provides: {"LAST_RTL_MASK"}
// Dependencies: {}
pub const LAST_RTL_MASK : BidiClassMask = BidiClassMask (bidi_class_to_mask (icu_properties :: props :: BidiClass :: RightToLeft) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: ArabicLetter) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: EuropeanNumber) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: ArabicNumber) ,) ;
};
}
