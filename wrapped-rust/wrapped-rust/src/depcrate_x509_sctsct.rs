// Generated macro for Sct (struct)
macro_rules! Depcrate_x509_sctSct {
() => {
// Module: crate::x509::sct
// Provides: {"Sct"}
// Dependencies: {}
# [pyo3 :: pyclass (frozen , module = "cryptography.hazmat.bindings._rust.x509")] pub (crate) struct Sct { log_id : [u8 ; 32] , timestamp : u64 , entry_type : LogEntryType , hash_algorithm : HashAlgorithm , signature_algorithm : SignatureAlgorithm , signature : Vec < u8 > , extension_bytes : Vec < u8 > , pub (crate) sct_data : Vec < u8 > , }
};
}
