// Generated macro for impl_312 (impl)
macro_rules! Depcrate_backend_poly1305impl_312 {
() => {
// Module: crate::backend::poly1305
// Provides: {"impl_312"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] impl Poly1305Open { fn new (key : CffiBuf < '_ >) -> CryptographyResult < Poly1305Open > { if cryptography_openssl :: fips :: is_enabled () { return Err (CryptographyError :: from (exceptions :: UnsupportedAlgorithm :: new_err (("poly1305 is not supported by this version of OpenSSL." , exceptions :: Reasons :: UNSUPPORTED_MAC ,)) ,)) ; } let pkey = openssl :: pkey :: PKey :: private_key_from_raw_bytes (key . as_bytes () , openssl :: pkey :: Id :: POLY1305 ,) . map_err (| _ | pyo3 :: exceptions :: PyValueError :: new_err ("A poly1305 key is 32 bytes long")) ? ; Ok (Poly1305Open { signer : openssl :: sign :: Signer :: new_without_digest (& pkey) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("A poly1305 key is 32 bytes long") }) ? , }) } fn update (& mut self , data : CffiBuf < '_ >) -> CryptographyResult < () > { let buf = data . as_bytes () ; self . signer . update (buf) ? ; Ok (()) } fn finalize < 'p > (& mut self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let result = pyo3 :: types :: PyBytes :: new_with (py , self . signer . len () ? , | b | { let n = self . signer . sign (b) . unwrap () ; assert_eq ! (n , b . len ()) ; Ok (()) }) ? ; Ok (result) } }
};
}
