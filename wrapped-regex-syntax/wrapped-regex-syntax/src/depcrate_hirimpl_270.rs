// Generated macro for impl_270 (impl)
macro_rules! Depcrate_hirimpl_270 {
() => {
// Module: crate::hir
// Provides: {"impl_270"}
// Dependencies: {}
impl ClassUnicodeRange { # [doc = " Create a new Unicode scalar value range for a character class."] # [doc = ""] # [doc = " The returned range is always in a canonical form. That is, the range"] # [doc = " returned always satisfies the invariant that `start <= end`."] pub fn new (start : char , end : char) -> ClassUnicodeRange { ClassUnicodeRange :: create (start , end) } # [doc = " Return the start of this range."] # [doc = ""] # [doc = " The start of a range is always less than or equal to the end of the"] # [doc = " range."] pub fn start (& self) -> char { self . start } # [doc = " Return the end of this range."] # [doc = ""] # [doc = " The end of a range is always greater than or equal to the start of the"] # [doc = " range."] pub fn end (& self) -> char { self . end } # [doc = " Returns the number of codepoints in this range."] pub fn len (& self) -> usize { let diff = 1 + u32 :: from (self . end) - u32 :: from (self . start) ; usize :: try_from (diff) . expect ("char class len fits in usize") } }
};
}
