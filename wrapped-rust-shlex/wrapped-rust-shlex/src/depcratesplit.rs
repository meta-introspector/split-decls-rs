// Generated macro for split (function)
macro_rules! Depcratesplit {
() => {
// Module: crate
// Provides: {"split"}
// Dependencies: {}
# [doc = " Convenience function that consumes the whole string at once.  Returns None if the input was"] # [doc = " erroneous."] pub fn split (in_str : & str) -> Option < Vec < String > > { let mut shl = Shlex :: new (in_str) ; let res = shl . by_ref () . collect () ; if shl . had_error { None } else { Some (res) } }
};
}
