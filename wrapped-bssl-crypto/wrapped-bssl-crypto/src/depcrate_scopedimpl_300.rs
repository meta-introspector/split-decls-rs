// Generated macro for impl_300 (impl)
macro_rules! Depcrate_scopedimpl_300 {
() => {
// Module: crate::scoped
// Provides: {"impl_300"}
// Dependencies: {}
impl Bignum { pub fn from_u64 (value : u64) -> Self { let mut ret = Bignum (unsafe { initialized_struct (| ptr | bssl_sys :: BN_init (ptr)) }) ; assert_eq ! (1 , unsafe { bssl_sys :: BN_set_u64 (& mut ret . 0 , value) }) ; ret } pub fn as_ffi_ptr (& self) -> * const bssl_sys :: BIGNUM { & self . 0 } }
};
}
