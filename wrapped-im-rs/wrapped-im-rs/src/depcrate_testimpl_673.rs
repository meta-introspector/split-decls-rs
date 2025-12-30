// Generated macro for impl_673 (impl)
macro_rules! Depcrate_testimpl_673 {
() => {
// Module: crate::test
// Provides: {"impl_673"}
// Dependencies: {}
impl < N : Unsigned > LolHasher < N > { fn feed_me (& mut self , byte : u8) { self . state ^= u64 :: from (byte) << self . shift ; self . shift += 8 ; if self . shift >= 64 { self . shift = 0 ; } } }
};
}
