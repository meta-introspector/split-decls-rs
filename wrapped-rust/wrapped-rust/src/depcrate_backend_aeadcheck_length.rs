// Generated macro for check_length (function)
macro_rules! Depcrate_backend_aeadcheck_length {
() => {
// Module: crate::backend::aead
// Provides: {"check_length"}
// Dependencies: {}
fn check_length (data : & [u8]) -> CryptographyResult < () > { if data . len () > (i32 :: MAX as usize) { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyOverflowError :: new_err ("Data or associated data too long. Max 2**31 - 1 bytes" ,) ,)) ; } Ok (()) }
};
}
