// Generated macro for RegistryCipher (enum)
macro_rules! Depcrate_backend_cipher_registryRegistryCipher {
() => {
// Module: crate::backend::cipher_registry
// Provides: {"RegistryCipher"}
// Dependencies: {}
enum RegistryCipher { Ref (& 'static openssl :: cipher :: CipherRef) , Owned (Cipher) , }
};
}
