// Generated macro for impl_310 (impl)
macro_rules! Depcrate_backend_poly1305impl_310 {
() => {
// Module: crate::backend::poly1305
// Provides: {"impl_310"}
// Dependencies: {}
# [cfg (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_AWSLC))] impl Poly1305Boring { fn new (key : CffiBuf < '_ >) -> CryptographyResult < Poly1305Boring > { if key . as_bytes () . len () != 32 { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("A poly1305 key is 32 bytes long") ,)) ; } let ctx = cryptography_openssl :: poly1305 :: Poly1305State :: new (key . as_bytes ()) ; Ok (Poly1305Boring { context : ctx }) } fn update (& mut self , data : CffiBuf < '_ >) -> CryptographyResult < () > { self . context . update (data . as_bytes ()) ; Ok (()) } fn finalize < 'p > (& mut self , py : pyo3 :: Python < 'p > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: types :: PyBytes > > { let result = pyo3 :: types :: PyBytes :: new_with (py , 16usize , | b | { self . context . finalize (b . as_mut ()) ; Ok (()) }) ? ; Ok (result) } }
};
}
