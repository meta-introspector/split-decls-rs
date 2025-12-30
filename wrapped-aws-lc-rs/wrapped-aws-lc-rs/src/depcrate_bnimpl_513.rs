// Generated macro for impl_513 (impl)
macro_rules! Depcrate_bnimpl_513 {
() => {
// Module: crate::bn
// Provides: {"impl_513"}
// Dependencies: {}
impl ConstPointer < '_ , BIGNUM > { pub (crate) fn to_be_bytes (& self) -> Vec < u8 > { unsafe { let bn_bytes = BN_num_bytes (* * self) ; let mut byte_vec = Vec :: with_capacity (bn_bytes as usize) ; let out_bytes = BN_bn2bin (* * self , byte_vec . as_mut_ptr ()) ; debug_assert_eq ! (out_bytes , bn_bytes as usize) ; byte_vec . set_len (out_bytes) ; byte_vec } } }
};
}
