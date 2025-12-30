// Generated macro for split (function)
macro_rules! Depcrate_bytessplit {
() => {
// Module: crate::bytes
// Provides: {"split"}
// Dependencies: {}
# [doc = " Convenience function that consumes the whole byte string at once.  Returns None if the input was"] # [doc = " erroneous."] pub fn split (in_bytes : & [u8]) -> Option < Vec < Vec < u8 > > > { let mut shl = Shlex :: new (in_bytes) ; let res = shl . by_ref () . collect () ; if shl . had_error { None } else { Some (res) } }
};
}
