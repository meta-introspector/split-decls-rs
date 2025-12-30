// Generated macro for parse_scts (function)
macro_rules! Depcrate_x509_sctparse_scts {
() => {
// Module: crate::x509::sct
// Provides: {"parse_scts"}
// Dependencies: {}
pub (crate) fn parse_scts < 'p > (py : pyo3 :: Python < 'p > , data : & [u8] , entry_type : LogEntryType ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let mut reader = TLSReader :: new (data) . read_length_prefixed () ? ; let py_scts = pyo3 :: types :: PyList :: empty (py) ; while ! reader . is_empty () { let mut sct_data = reader . read_length_prefixed () ? ; let raw_sct_data = sct_data . data . to_vec () ; let version = sct_data . read_byte () ? ; if version != 0 { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid SCT version") ,)) ; } let log_id = sct_data . read_exact (32) ? . try_into () . unwrap () ; let timestamp = u64 :: from_be_bytes (sct_data . read_exact (8) ? . try_into () . unwrap ()) ; let extension_bytes = sct_data . read_length_prefixed () ? . data . to_vec () ; let hash_algorithm = sct_data . read_byte () ? . try_into () ? ; let signature_algorithm = sct_data . read_byte () ? . try_into () ? ; let signature = sct_data . read_length_prefixed () ? . data . to_vec () ; let sct = Sct { log_id , timestamp , entry_type : entry_type . clone () , hash_algorithm , signature_algorithm , signature , extension_bytes , sct_data : raw_sct_data , } ; py_scts . append (pyo3 :: Bound :: new (py , sct) ?) ? ; } Ok (py_scts . into_any ()) }
};
}
