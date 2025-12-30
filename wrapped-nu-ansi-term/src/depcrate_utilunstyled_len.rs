// Generated macro for unstyled_len (function)
macro_rules! Depcrate_utilunstyled_len {
() => {
// Module: crate::util
// Provides: {"unstyled_len"}
// Dependencies: {}
# [doc = " Return the unstyled length of AnsiStrings. This is equaivalent to `unstyle(strs).len()`."] pub fn unstyled_len (strs : & AnsiStrings) -> usize { let mut l = 0 ; for i in strs . 0 . iter () { l += i . string . len () ; } l }
};
}
