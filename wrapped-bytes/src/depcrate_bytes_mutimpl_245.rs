// Generated macro for impl_245 (impl)
macro_rules! Depcrate_bytes_mutimpl_245 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_245"}
// Dependencies: {}
impl Buf for BytesMut { # [inline] fn remaining (& self) -> usize { self . len () } # [inline] fn chunk (& self) -> & [u8] { self . as_slice () } # [inline] fn advance (& mut self , cnt : usize) { assert ! (cnt <= self . remaining () , "cannot advance past `remaining`: {:?} <= {:?}" , cnt , self . remaining () ,) ; unsafe { self . advance_unchecked (cnt) ; } } fn copy_to_bytes (& mut self , len : usize) -> Bytes { self . split_to (len) . freeze () } }
};
}
