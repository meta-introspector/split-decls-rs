// Generated macro for LazyEvpCipherAead (struct)
macro_rules! Depcrate_backend_aeadLazyEvpCipherAead {
() => {
// Module: crate::backend::aead
// Provides: {"LazyEvpCipherAead"}
// Dependencies: {}
struct LazyEvpCipherAead { cipher : & 'static openssl :: cipher :: CipherRef , key : pyo3 :: Py < pyo3 :: PyAny > , tag_len : usize , tag_first : bool , is_ccm : bool , }
};
}
