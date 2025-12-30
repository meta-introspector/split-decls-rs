// Generated macro for impl_364 (impl)
macro_rules! Depcrate_util_alphabetimpl_364 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_364"}
// Dependencies: {}
impl ByteSet { # [doc = " Create an empty set of bytes."] pub (crate) fn empty () -> ByteSet { ByteSet { bits : BitSet ([0 ; 2]) } } # [doc = " Add a byte to this set."] # [doc = ""] # [doc = " If the given byte already belongs to this set, then this is a no-op."] pub (crate) fn add (& mut self , byte : u8) { let bucket = byte / 128 ; let bit = byte % 128 ; self . bits . 0 [usize :: from (bucket)] |= 1 << bit ; } # [doc = " Return true if and only if the given byte is in this set."] pub (crate) fn contains (& self , byte : u8) -> bool { let bucket = byte / 128 ; let bit = byte % 128 ; self . bits . 0 [usize :: from (bucket)] & (1 << bit) > 0 } }
};
}
