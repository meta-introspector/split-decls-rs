// Generated macro for impl_277 (impl)
macro_rules! Depcrate_hirimpl_277 {
() => {
// Module: crate::hir
// Provides: {"impl_277"}
// Dependencies: {}
impl ClassBytesRange { # [doc = " Create a new byte range for a character class."] # [doc = ""] # [doc = " The returned range is always in a canonical form. That is, the range"] # [doc = " returned always satisfies the invariant that `start <= end`."] pub fn new (start : u8 , end : u8) -> ClassBytesRange { ClassBytesRange :: create (start , end) } # [doc = " Return the start of this range."] # [doc = ""] # [doc = " The start of a range is always less than or equal to the end of the"] # [doc = " range."] pub fn start (& self) -> u8 { self . start } # [doc = " Return the end of this range."] # [doc = ""] # [doc = " The end of a range is always greater than or equal to the start of the"] # [doc = " range."] pub fn end (& self) -> u8 { self . end } # [doc = " Returns the number of bytes in this range."] pub fn len (& self) -> usize { usize :: from (self . end . checked_sub (self . start) . unwrap ()) . checked_add (1) . unwrap () } }
};
}
