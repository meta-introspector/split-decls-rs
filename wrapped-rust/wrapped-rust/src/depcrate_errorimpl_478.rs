// Generated macro for impl_478 (impl)
macro_rules! Depcrate_errorimpl_478 {
() => {
// Module: crate::error
// Provides: {"impl_478"}
// Dependencies: {}
# [pyo3 :: pymethods] impl OpenSSLError { # [getter] fn lib (& self) -> i32 { self . e . library_code () } # [getter] fn reason (& self) -> i32 { self . e . reason_code () } # [getter] fn reason_text (& self) -> & [u8] { self . e . reason () . unwrap_or ("") . as_bytes () } fn __repr__ (& self) -> pyo3 :: PyResult < String > { Ok (format ! ("<OpenSSLError(code={}, lib={}, reason={}, reason_text={})>" , self . e . code () , self . e . library_code () , self . e . reason_code () , self . e . reason () . unwrap_or (""))) } }
};
}
