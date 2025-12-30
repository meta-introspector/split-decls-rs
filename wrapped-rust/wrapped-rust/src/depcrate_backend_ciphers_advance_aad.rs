// Generated macro for _advance_aad (function)
macro_rules! Depcrate_backend_ciphers_advance_aad {
() => {
// Module: crate::backend::ciphers
// Provides: {"_advance_aad"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn _advance_aad (ctx : pyo3 :: Bound < '_ , pyo3 :: PyAny > , n : u64) { if let Ok (c) = ctx . cast :: < PyAEADEncryptionContext > () { c . borrow_mut () . aad_bytes_remaining -= n ; } else if let Ok (c) = ctx . cast :: < PyAEADDecryptionContext > () { c . borrow_mut () . aad_bytes_remaining -= n ; } }
};
}
