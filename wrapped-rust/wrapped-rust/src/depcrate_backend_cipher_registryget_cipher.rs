// Generated macro for get_cipher (function)
macro_rules! Depcrate_backend_cipher_registryget_cipher {
() => {
// Module: crate::backend::cipher_registry
// Provides: {"get_cipher"}
// Dependencies: {}
pub (crate) fn get_cipher < 'py > (py : pyo3 :: Python < 'py > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > , mode_cls : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < Option < & 'py openssl :: cipher :: CipherRef > > { let registry = get_cipher_registry (py) ? ; let key_size = algorithm . getattr (pyo3 :: intern ! (py , "key_size")) ? . extract () ? ; let key = RegistryKey :: new (py , algorithm . get_type () . into () , mode_cls . into () , key_size) ? ; match registry . get (& key) { Some (RegistryCipher :: Ref (c)) => Ok (Some (c)) , Some (RegistryCipher :: Owned (c)) => Ok (Some (c)) , None => Ok (None) , } }
};
}
