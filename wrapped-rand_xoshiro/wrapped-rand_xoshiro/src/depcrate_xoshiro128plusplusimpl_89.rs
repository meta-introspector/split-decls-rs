// Generated macro for impl_89 (impl)
macro_rules! Depcrate_xoshiro128plusplusimpl_89 {
() => {
// Module: crate::xoshiro128plusplus
// Provides: {"impl_89"}
// Dependencies: {}
impl RngCore for Xoshiro128PlusPlus { # [inline] fn next_u32 (& mut self) -> u32 { let result_starstar = plusplus_u32 ! (self . s [0] , self . s [3]) ; impl_xoshiro_u32 ! (self) ; result_starstar } # [inline] fn next_u64 (& mut self) -> u64 { next_u64_via_u32 (self) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
