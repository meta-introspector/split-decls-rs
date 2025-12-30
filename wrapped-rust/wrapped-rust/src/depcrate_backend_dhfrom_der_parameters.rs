// Generated macro for from_der_parameters (function)
macro_rules! Depcrate_backend_dhfrom_der_parameters {
() => {
// Module: crate::backend::dh
// Provides: {"from_der_parameters"}
// Dependencies: {}
# [pyo3 :: pyfunction] # [pyo3 (signature = (data , backend = None))] fn from_der_parameters (data : & [u8] , backend : Option < pyo3 :: Bound < '_ , pyo3 :: PyAny > > ,) -> CryptographyResult < DHParameters > { let _ = backend ; let asn1_params = asn1 :: parse_single :: < common :: DHParams < '_ > > (data) ? ; let p = openssl :: bn :: BigNum :: from_slice (asn1_params . p . as_bytes ()) ? ; let q = asn1_params . q . map (| q | openssl :: bn :: BigNum :: from_slice (q . as_bytes ())) . transpose () ? ; let g = openssl :: bn :: BigNum :: from_slice (asn1_params . g . as_bytes ()) ? ; Ok (DHParameters { dh : openssl :: dh :: Dh :: from_pqg (p , q , g) ? , }) }
};
}
