// Generated macro for impl_20 (impl)
macro_rules! Depcrate_blockimpl_20 {
() => {
// Module: crate::block
// Provides: {"impl_20"}
// Dependencies: {}
impl < Alg : BlockCipherDecrypt > BlockCipherDecrypt for & Alg { fn decrypt_with_backend (& self , f : impl BlockCipherDecClosure < BlockSize = Self :: BlockSize >) { Alg :: decrypt_with_backend (self , f) ; } }
};
}
