// Generated macro for encode_scts (function)
macro_rules! Depcrate_x509_extensionsencode_scts {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_scts"}
// Dependencies: {}
fn encode_scts (ext : & pyo3 :: Bound < '_ , pyo3 :: PyAny >) -> CryptographyResult < Vec < u8 > > { let mut length = 0 ; for sct in ext . try_iter () ? { let sct = sct ? . cast :: < sct :: Sct > () ? . clone () ; length += sct . get () . sct_data . len () + 2 ; } let mut result = vec ! [] ; result . extend_from_slice (& (length as u16) . to_be_bytes ()) ; for sct in ext . try_iter () ? { let sct = sct ? . cast :: < sct :: Sct > () ? . clone () ; result . extend_from_slice (& (sct . get () . sct_data . len () as u16) . to_be_bytes ()) ; result . extend_from_slice (& sct . get () . sct_data) ; } Ok (asn1 :: write_single (& result . as_slice ()) ?) }
};
}
