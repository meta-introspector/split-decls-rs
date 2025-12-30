// Generated macro for impl_249 (impl)
macro_rules! Depcrate_hkdfimpl_249 {
() => {
// Module: crate::hkdf
// Provides: {"impl_249"}
// Dependencies: {}
impl PrkMode { fn fill (& self , algorithm : Algorithm , out : & mut [u8] , info : & [u8]) -> Result < () , Unspecified > { let digest = * digest :: match_digest_type (& algorithm . 0 . digest_algorithm () . id) ; match & self { PrkMode :: Expand { key_bytes , key_len } => unsafe { if 1 != indicator_check ! (HKDF_expand (out . as_mut_ptr () , out . len () , digest , key_bytes . as_ptr () , * key_len , info . as_ptr () , info . len () ,)) { return Err (Unspecified) ; } } , PrkMode :: ExtractExpand { secret , salt , salt_len , } => { if 1 != indicator_check ! (unsafe { HKDF (out . as_mut_ptr () , out . len () , digest , secret . as_ptr () , secret . len () , salt . as_ptr () , * salt_len , info . as_ptr () , info . len () ,) }) { return Err (Unspecified) ; } } } Ok (()) } }
};
}
