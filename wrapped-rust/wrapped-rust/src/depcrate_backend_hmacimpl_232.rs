// Generated macro for impl_232 (impl)
macro_rules! Depcrate_backend_hmacimpl_232 {
() => {
// Module: crate::backend::hmac
// Provides: {"impl_232"}
// Dependencies: {}
impl Hmac { pub (crate) fn new_bytes (py : pyo3 :: Python < '_ > , key : & [u8] , algorithm : & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> CryptographyResult < Hmac > { let md = message_digest_from_algorithm (py , algorithm) ? ; let ctx = cryptography_openssl :: hmac :: Hmac :: new (key , md) . map_err (| _ | { exceptions :: UnsupportedAlgorithm :: new_err (("Digest is not supported for HMAC" , exceptions :: Reasons :: UNSUPPORTED_HASH ,)) }) ? ; Ok (Hmac { ctx : Some (ctx) , algorithm : algorithm . clone () . unbind () , }) } pub (crate) fn update_bytes (& mut self , data : & [u8]) -> CryptographyResult < () > { self . get_mut_ctx () ? . update (data) ? ; Ok (()) } pub (crate) fn finalize_bytes (& mut self ,) -> CryptographyResult < cryptography_openssl :: hmac :: DigestBytes > { let data = self . get_mut_ctx () ? . finish () ? ; self . ctx = None ; Ok (data) } fn get_ctx (& self) -> CryptographyResult < & cryptography_openssl :: hmac :: Hmac > { if let Some (ctx) = self . ctx . as_ref () { return Ok (ctx) ; } ; Err (exceptions :: already_finalized_error ()) } fn get_mut_ctx (& mut self) -> CryptographyResult < & mut cryptography_openssl :: hmac :: Hmac > { if let Some (ctx) = self . ctx . as_mut () { return Ok (ctx) ; } Err (exceptions :: already_finalized_error ()) } }
};
}
