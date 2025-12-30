// Generated macro for impl_123 (impl)
macro_rules! Depcrate_bytesimpl_123 {
() => {
// Module: crate::bytes
// Provides: {"impl_123"}
// Dependencies: {}
impl Buf for Bytes { # [inline] fn remaining (& self) -> usize { self . len () } # [inline] fn chunk (& self) -> & [u8] { self . as_slice () } # [inline] fn advance (& mut self , cnt : usize) { assert ! (cnt <= self . len () , "cannot advance past `remaining`: {:?} <= {:?}" , cnt , self . len () ,) ; unsafe { self . inc_start (cnt) ; } } fn copy_to_bytes (& mut self , len : usize) -> Self { self . split_to (len) } }
};
}
