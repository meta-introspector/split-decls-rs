// Generated macro for parse_crl_reason_flags (function)
macro_rules! Depcrate_x509_crlparse_crl_reason_flags {
() => {
// Module: crate::x509::crl
// Provides: {"parse_crl_reason_flags"}
// Dependencies: {}
pub (crate) fn parse_crl_reason_flags < 'p > (py : pyo3 :: Python < 'p > , reason : & crl :: CRLReason ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let flag_name = match reason . value () { 0 => "unspecified" , 1 => "key_compromise" , 2 => "ca_compromise" , 3 => "affiliation_changed" , 4 => "superseded" , 5 => "cessation_of_operation" , 6 => "certificate_hold" , 8 => "remove_from_crl" , 9 => "privilege_withdrawn" , 10 => "aa_compromise" , value => { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (format ! ("Unsupported reason code: {value}")) ,)) } } ; Ok (types :: REASON_FLAGS . get (py) ? . getattr (flag_name) ?) }
};
}
