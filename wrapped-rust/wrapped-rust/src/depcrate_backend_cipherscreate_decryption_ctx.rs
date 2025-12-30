// Generated macro for create_decryption_ctx (function)
macro_rules! Depcrate_backend_cipherscreate_decryption_ctx {
() => {
// Module: crate::backend::ciphers
// Provides: {"create_decryption_ctx"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn create_decryption_ctx < 'p > (py : pyo3 :: Python < 'p > , algorithm : pyo3 :: Bound < '_ , pyo3 :: PyAny > , mode : pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let mut ctx = CipherContext :: new (py , algorithm , mode . clone () , openssl :: symm :: Mode :: Decrypt) ? ; if mode . is_instance (& types :: MODE_WITH_AUTHENTICATION_TAG . get (py) ?) ? { if let Some (tag) = mode . getattr (pyo3 :: intern ! (py , "tag")) ? . extract :: < Option < pyo3 :: pybacked :: PyBackedBytes > > () ? { ctx . ctx . set_tag (& tag) ? ; } Ok (PyAEADDecryptionContext { ctx : Some (ctx) , updated : false , bytes_remaining : mode . getattr (pyo3 :: intern ! (py , "_MAX_ENCRYPTED_BYTES")) ? . extract () ? , aad_bytes_remaining : mode . getattr (pyo3 :: intern ! (py , "_MAX_AAD_BYTES")) ? . extract () ? , } . into_pyobject (py) ? . into_any ()) } else { Ok (PyCipherContext { ctx : Some (ctx) } . into_pyobject (py) ? . into_any ()) } }
};
}
