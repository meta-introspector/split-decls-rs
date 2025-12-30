// Generated macro for bidi_class_to_mask (function)
macro_rules! Depcratebidi_class_to_mask {
() => {
// Module: crate
// Provides: {"bidi_class_to_mask"}
// Dependencies: {}
# [doc = " Turns a bidi class into a mask for comparing with multiple classes at once."] const fn bidi_class_to_mask (bc : icu_properties :: props :: BidiClass) -> u32 { 1u32 << bc . to_icu4c_value () }
};
}
