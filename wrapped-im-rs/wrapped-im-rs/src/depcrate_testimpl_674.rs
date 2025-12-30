// Generated macro for impl_674 (impl)
macro_rules! Depcrate_testimpl_674 {
() => {
// Module: crate::test
// Provides: {"impl_674"}
// Dependencies: {}
impl < N : Unsigned > Hasher for LolHasher < N > { fn write (& mut self , bytes : & [u8]) { for byte in bytes { self . feed_me (* byte) } } fn finish (& self) -> u64 { if N :: USIZE == 64 { self . state } else { self . state & ((1 << N :: USIZE) - 1) } } }
};
}
