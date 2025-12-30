// Generated macro for MIDDLE_RTL_MASK (const)
macro_rules! DepcrateMIDDLE_RTL_MASK {
() => {
// Module: crate
// Provides: {"MIDDLE_RTL_MASK"}
// Dependencies: {}
pub const MIDDLE_RTL_MASK : BidiClassMask = BidiClassMask (bidi_class_to_mask (icu_properties :: props :: BidiClass :: RightToLeft) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: ArabicLetter) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: ArabicNumber) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: EuropeanNumber) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: EuropeanSeparator) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: CommonSeparator) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: EuropeanTerminator) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: OtherNeutral) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: BoundaryNeutral) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: NonspacingMark) ,) ;
};
}
