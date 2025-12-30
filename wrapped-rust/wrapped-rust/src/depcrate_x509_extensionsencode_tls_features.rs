// Generated macro for encode_tls_features (function)
macro_rules! Depcrate_x509_extensionsencode_tls_features {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_tls_features"}
// Dependencies: {}
fn encode_tls_features (py : pyo3 :: Python < '_ > , ext : & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < Vec < u8 > > { let mut els = vec ! [] ; for el in ext . try_iter () ? { els . push (el ? . getattr (pyo3 :: intern ! (py , "value")) ? . extract :: < u64 > () ?) ; } Ok (asn1 :: write_single (& asn1 :: SequenceOfWriter :: new (els)) ?) }
};
}
