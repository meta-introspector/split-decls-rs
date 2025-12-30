// Generated macro for MIDDLE_LTR_MASK (const)
macro_rules! DepcrateMIDDLE_LTR_MASK {
() => {
// Module: crate
// Provides: {"MIDDLE_LTR_MASK"}
// Dependencies: {}
pub const MIDDLE_LTR_MASK : BidiClassMask = BidiClassMask (bidi_class_to_mask (icu_properties :: props :: BidiClass :: LeftToRight) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: EuropeanNumber) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: EuropeanSeparator) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: CommonSeparator) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: EuropeanTerminator) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: OtherNeutral) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: BoundaryNeutral) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: NonspacingMark) ,) ;
};
}
