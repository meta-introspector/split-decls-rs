// Generated macro for impl_59 (impl)
macro_rules! Depcrate_backend_cipher_registryimpl_59 {
() => {
// Module: crate::backend::cipher_registry
// Provides: {"impl_59"}
// Dependencies: {}
impl std :: hash :: Hash for RegistryKey { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . algorithm_hash . hash (state) ; self . mode_hash . hash (state) ; } }
};
}
