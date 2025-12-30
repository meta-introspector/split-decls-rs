// Generated macro for char_from_codepoint (function)
macro_rules! Depcrate_scannerschar_from_codepoint {
() => {
// Module: crate::scanners
// Provides: {"char_from_codepoint"}
// Dependencies: {}
fn char_from_codepoint (input : usize) -> Option < char > { let codepoint = input . try_into () . ok () ? ; if codepoint == 0 { return None ; } char :: from_u32 (codepoint) }
};
}
