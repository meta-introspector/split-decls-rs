// Generated macro for impl_145 (impl)
macro_rules! Depcrate_xoshiro512plusplusimpl_145 {
() => {
// Module: crate::xoshiro512plusplus
// Provides: {"impl_145"}
// Dependencies: {}
impl RngCore for Xoshiro512PlusPlus { # [inline] fn next_u32 (& mut self) -> u32 { (self . next_u64 () >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let result_plusplus = plusplus_u64 ! (self . s [2] , self . s [0] , 17) ; impl_xoshiro_large ! (self) ; result_plusplus } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
