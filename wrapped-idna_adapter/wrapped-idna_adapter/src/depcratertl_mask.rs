// Generated macro for RTL_MASK (const)
macro_rules! DepcrateRTL_MASK {
() => {
// Module: crate
// Provides: {"RTL_MASK"}
// Dependencies: {}
# [doc = " Mask for checking if the domain is a bidi domain."] pub const RTL_MASK : BidiClassMask = BidiClassMask (bidi_class_to_mask (icu_properties :: props :: BidiClass :: RightToLeft) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: ArabicLetter) | bidi_class_to_mask (icu_properties :: props :: BidiClass :: ArabicNumber) ,) ;
};
}
