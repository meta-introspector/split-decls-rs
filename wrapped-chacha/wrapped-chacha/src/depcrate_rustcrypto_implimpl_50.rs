// Generated macro for impl_50 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_50 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_50"}
// Dependencies: {}
impl < NonceSize : Unsigned , Rounds , IsX > ChaChaAny < NonceSize , Rounds , IsX > { # [inline (always)] fn seek (& mut self , ct : u64) { if NonceSize :: U32 != 12 { seek64 (& mut self . state , ct) ; } else { seek32 (& mut self . state , ct) ; } } }
};
}
