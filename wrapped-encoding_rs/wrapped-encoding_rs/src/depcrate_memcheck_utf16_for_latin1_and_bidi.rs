// Generated macro for check_utf16_for_latin1_and_bidi (function)
macro_rules! Depcrate_memcheck_utf16_for_latin1_and_bidi {
() => {
// Module: crate::mem
// Provides: {"check_utf16_for_latin1_and_bidi"}
// Dependencies: {}
# [doc = " Checks whether a potentially invalid UTF-16 buffer contains code points"] # [doc = " that trigger right-to-left processing or is all-Latin1."] # [doc = ""] # [doc = " Possibly more efficient than performing the checks separately."] # [doc = ""] # [doc = " Returns `Latin1Bidi::Latin1` if `is_utf16_latin1()` would return `true`."] # [doc = " Otherwise, returns `Latin1Bidi::Bidi` if `is_utf16_bidi()` would return"] # [doc = " `true`. Otherwise, returns `Latin1Bidi::LeftToRight`."] pub fn check_utf16_for_latin1_and_bidi (buffer : & [u16]) -> Latin1Bidi { check_utf16_for_latin1_and_bidi_impl (buffer) }
};
}
