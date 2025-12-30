// Generated macro for _advance (function)
macro_rules! Depcrate_backend_ciphers_advance {
() => {
// Module: crate::backend::ciphers
// Provides: {"_advance"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn _advance (ctx : pyo3 :: Bound < '_ , pyo3 :: PyAny > , n : u64) { if let Ok (c) = ctx . cast :: < PyAEADEncryptionContext > () { c . borrow_mut () . bytes_remaining -= n ; } else if let Ok (c) = ctx . cast :: < PyAEADDecryptionContext > () { c . borrow_mut () . bytes_remaining -= n ; } }
};
}
