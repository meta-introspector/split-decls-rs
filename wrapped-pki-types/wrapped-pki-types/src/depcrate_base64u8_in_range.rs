// Generated macro for u8_in_range (function)
macro_rules! Depcrate_base64u8_in_range {
() => {
// Module: crate::base64
// Provides: {"u8_in_range"}
// Dependencies: {}
# [doc = " Returns 0xff if `a` in `lo..=hi`."] # [doc = ""] # [doc = " lo..=hi must not be 0..=255.  Callers in this file have constant"] # [doc = " `lo` and `hi`, and this function is private to this file."] fn u8_in_range (a : u8 , lo : u8 , hi : u8) -> u8 { debug_assert ! (lo <= hi) ; debug_assert ! (hi - lo != 255) ; let a = a . wrapping_sub (lo) ; u8_less_than (a , (hi - lo) . wrapping_add (1)) }
};
}
