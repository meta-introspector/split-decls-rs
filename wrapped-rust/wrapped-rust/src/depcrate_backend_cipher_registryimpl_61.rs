// Generated macro for impl_61 (impl)
macro_rules! Depcrate_backend_cipher_registryimpl_61 {
() => {
// Module: crate::backend::cipher_registry
// Provides: {"impl_61"}
// Dependencies: {}
impl From < & 'static openssl :: cipher :: CipherRef > for RegistryCipher { fn from (c : & 'static openssl :: cipher :: CipherRef) -> RegistryCipher { RegistryCipher :: Ref (c) } }
};
}
