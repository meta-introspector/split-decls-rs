// Generated macro for encode_private_key_usage_period (function)
macro_rules! Depcrate_x509_extensionsencode_private_key_usage_period {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_private_key_usage_period"}
// Dependencies: {}
pub (crate) fn encode_private_key_usage_period (py : pyo3 :: Python < '_ > , ext : & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < Vec < u8 > > { let not_before = ext . getattr (pyo3 :: intern ! (py , "not_before")) ? . extract () ? ; let not_after = ext . getattr (pyo3 :: intern ! (py , "not_after")) ? . extract () ? ; let not_before_value = if let Some (not_before) = not_before { let dt = x509 :: py_to_datetime (py , not_before) ? ; Some (asn1 :: X509GeneralizedTime :: new (dt) ?) } else { None } ; let not_after_value = if let Some (not_after) = not_after { let dt = x509 :: py_to_datetime (py , not_after) ? ; Some (asn1 :: X509GeneralizedTime :: new (dt) ?) } else { None } ; let pkup = extensions :: PrivateKeyUsagePeriod { not_before : not_before_value , not_after : not_after_value , } ; Ok (asn1 :: write_single (& pkup) ?) }
};
}
