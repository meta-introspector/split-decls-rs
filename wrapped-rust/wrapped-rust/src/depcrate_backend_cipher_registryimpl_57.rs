// Generated macro for impl_57 (impl)
macro_rules! Depcrate_backend_cipher_registryimpl_57 {
() => {
// Module: crate::backend::cipher_registry
// Provides: {"impl_57"}
// Dependencies: {}
impl PartialEq for RegistryKey { fn eq (& self , other : & RegistryKey) -> bool { self . algorithm . is (& other . algorithm) && self . mode . is (& other . mode) && (self . key_size == other . key_size || self . key_size . is_none () || other . key_size . is_none ()) } }
};
}
