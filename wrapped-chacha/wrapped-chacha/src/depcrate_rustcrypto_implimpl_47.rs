// Generated macro for impl_47 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_47 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (test)] impl < NonceSize , Rounds : Unsigned , IsX > ChaChaAny < NonceSize , Rounds , IsX > { pub fn try_apply_keystream_narrow (& mut self , data : & mut [u8]) -> Result < () , () > { self . state . try_apply_keystream :: < WideDisabled > (data , Rounds :: U32) } }
};
}
