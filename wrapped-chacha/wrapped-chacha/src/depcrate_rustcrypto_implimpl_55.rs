// Generated macro for impl_55 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_55 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_55"}
// Dependencies: {}
impl < NonceSize , Rounds : Unsigned , IsX > StreamCipher for ChaChaAny < NonceSize , Rounds , IsX > { # [inline] fn try_apply_keystream (& mut self , data : & mut [u8]) -> Result < () , LoopError > { Self :: try_apply_keystream (self , data) . map_err (| _ | LoopError) } }
};
}
