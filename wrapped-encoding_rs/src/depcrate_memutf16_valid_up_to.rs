// Generated macro for utf16_valid_up_to (function)
macro_rules! Depcrate_memutf16_valid_up_to {
() => {
// Module: crate::mem
// Provides: {"utf16_valid_up_to"}
// Dependencies: {}
# [doc = " Returns the index of the first unpaired surrogate or, if the input is"] # [doc = " valid UTF-16 in its entirety, the length of the input."] pub fn utf16_valid_up_to (buffer : & [u16]) -> usize { utf16_valid_up_to_impl (buffer) }
};
}
