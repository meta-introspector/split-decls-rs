// Generated macro for impl_19 (impl)
macro_rules! Depcrate_blockimpl_19 {
() => {
// Module: crate::block
// Provides: {"impl_19"}
// Dependencies: {}
impl < Alg : BlockCipherEncrypt > BlockCipherEncrypt for & Alg { fn encrypt_with_backend (& self , f : impl BlockCipherEncClosure < BlockSize = Self :: BlockSize >) { Alg :: encrypt_with_backend (self , f) ; } }
};
}
