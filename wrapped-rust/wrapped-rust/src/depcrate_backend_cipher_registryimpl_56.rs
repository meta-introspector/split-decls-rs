// Generated macro for impl_56 (impl)
macro_rules! Depcrate_backend_cipher_registryimpl_56 {
() => {
// Module: crate::backend::cipher_registry
// Provides: {"impl_56"}
// Dependencies: {}
impl RegistryKey { fn new (py : pyo3 :: Python < '_ > , algorithm : pyo3 :: Py < pyo3 :: PyAny > , mode : pyo3 :: Py < pyo3 :: PyAny > , key_size : Option < u16 > ,) -> CryptographyResult < Self > { Ok (Self { algorithm : algorithm . clone_ref (py) , mode : mode . clone_ref (py) , key_size , algorithm_hash : algorithm . bind (py) . hash () ? , mode_hash : mode . bind (py) . hash () ? , }) } }
};
}
