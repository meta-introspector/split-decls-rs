// Generated macro for impl_64 (impl)
macro_rules! Depcrate_backend_cipher_registryimpl_64 {
() => {
// Module: crate::backend::cipher_registry
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'p > RegistryBuilder < 'p > { fn new (py : pyo3 :: Python < 'p >) -> Self { RegistryBuilder { py , m : HashMap :: new () , } } fn add (& mut self , algorithm : & pyo3 :: Bound < '_ , pyo3 :: PyAny > , mode : & pyo3 :: Bound < '_ , pyo3 :: PyAny > , key_size : Option < u16 > , cipher : impl Into < RegistryCipher > ,) -> CryptographyResult < () > { self . m . insert (RegistryKey :: new (self . py , algorithm . clone () . unbind () , mode . clone () . unbind () , key_size ,) ? , cipher . into () ,) ; Ok (()) } fn build (self) -> HashMap < RegistryKey , RegistryCipher > { self . m } }
};
}
