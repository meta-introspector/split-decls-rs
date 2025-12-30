// Generated macro for from_utf8_unchecked (function)
macro_rules! Depcrate_adapter_stripfrom_utf8_unchecked {
() => {
// Module: crate::adapter::strip
// Provides: {"from_utf8_unchecked"}
// Dependencies: {}
# [inline] unsafe fn from_utf8_unchecked < 'b > (bytes : & 'b [u8] , safety_justification : & 'static str) -> & 'b str { unsafe { if cfg ! (debug_assertions) { std :: str :: from_utf8 (bytes) . expect (safety_justification) } else { std :: str :: from_utf8_unchecked (bytes) } } }
};
}
