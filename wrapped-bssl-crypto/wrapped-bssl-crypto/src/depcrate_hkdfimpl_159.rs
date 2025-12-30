// Generated macro for impl_159 (impl)
macro_rules! Depcrate_hkdfimpl_159 {
() => {
// Module: crate::hkdf
// Provides: {"impl_159"}
// Dependencies: {}
impl Salt < '_ > { fn as_ffi_ptr (& self) -> * const u8 { match self { Salt :: None => core :: ptr :: null () , Salt :: NonEmpty (salt) => salt . as_ffi_ptr () , } } fn len (& self) -> usize { match self { Salt :: None => 0 , Salt :: NonEmpty (salt) => salt . len () , } } }
};
}
