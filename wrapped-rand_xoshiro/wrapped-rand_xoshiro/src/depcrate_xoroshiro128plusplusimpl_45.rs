// Generated macro for impl_45 (impl)
macro_rules! Depcrate_xoroshiro128plusplusimpl_45 {
() => {
// Module: crate::xoroshiro128plusplus
// Provides: {"impl_45"}
// Dependencies: {}
impl RngCore for Xoroshiro128PlusPlus { # [inline] fn next_u32 (& mut self) -> u32 { self . next_u64 () as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let r = plusplus_u64 ! (self . s0 , self . s1 , 17) ; impl_xoroshiro_u64_plusplus ! (self) ; r } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
