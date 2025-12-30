// Generated macro for LAST_LTR_MASK (const)
macro_rules! DepcrateLAST_LTR_MASK {
() => {
// Module: crate
// Provides: {"LAST_LTR_MASK"}
// Dependencies: {}
pub const LAST_LTR_MASK : BidiClassMask = BidiClassMask (bidi_class_to_mask (icu_properties :: props :: BidiClass :: LeftToRight) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: EuropeanNumber) ,) ;
};
}
