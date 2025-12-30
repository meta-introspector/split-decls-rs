// Generated macro for FIRST_BC_MASK (const)
macro_rules! DepcrateFIRST_BC_MASK {
() => {
// Module: crate
// Provides: {"FIRST_BC_MASK"}
// Dependencies: {}
# [doc = " Mask for allowable bidi classes in the first character of a label"] # [doc = " (either LTR or RTL) in a bidi domain."] pub const FIRST_BC_MASK : BidiClassMask = BidiClassMask (bidi_class_to_mask (icu_properties :: props :: BidiClass :: LeftToRight) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: RightToLeft) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: ArabicLetter) ,) ;
};
}
