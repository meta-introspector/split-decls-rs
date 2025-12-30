// Generated macro for impl_116 (impl)
macro_rules! Depcrate_xoshiro256plusplusimpl_116 {
() => {
// Module: crate::xoshiro256plusplus
// Provides: {"impl_116"}
// Dependencies: {}
impl RngCore for Xoshiro256PlusPlus { # [inline] fn next_u32 (& mut self) -> u32 { (self . next_u64 () >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let result_plusplus = plusplus_u64 ! (self . s [0] , self . s [3] , 23) ; impl_xoshiro_u64 ! (self) ; result_plusplus } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
