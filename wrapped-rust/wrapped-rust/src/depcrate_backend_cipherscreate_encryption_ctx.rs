// Generated macro for create_encryption_ctx (function)
macro_rules! Depcrate_backend_cipherscreate_encryption_ctx {
() => {
// Module: crate::backend::ciphers
// Provides: {"create_encryption_ctx"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn create_encryption_ctx < 'p > (py : pyo3 :: Python < 'p > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > , mode : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let ctx = CipherContext :: new (py , algorithm , mode . clone () , openssl :: symm :: Mode :: Encrypt) ? ; if mode . is_instance (& types :: MODE_WITH_AUTHENTICATION_TAG . get (py) ?) ? { Ok (PyAEADEncryptionContext { ctx : Some (ctx) , tag : None , updated : false , bytes_remaining : mode . getattr (pyo3 :: intern ! (py , "_MAX_ENCRYPTED_BYTES")) ? . extract () ? , aad_bytes_remaining : mode . getattr (pyo3 :: intern ! (py , "_MAX_AAD_BYTES")) ? . extract () ? , } . into_pyobject (py) ? . into_any ()) } else { Ok (PyCipherContext { ctx : Some (ctx) } . into_pyobject (py) ? . into_any ()) } }
};
}
