// Generated macro for impl_51 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_51 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_51"}
// Dependencies: {}
impl < NonceSize , Rounds : Unsigned , IsX > ChaChaAny < NonceSize , Rounds , IsX > { # [inline] fn try_apply_keystream (& mut self , data : & mut [u8]) -> Result < () , () > { self . state . try_apply_keystream :: < WideEnabled > (data , Rounds :: U32) } }
};
}
