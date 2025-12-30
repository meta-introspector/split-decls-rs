// Generated macro for impl_37 (impl)
macro_rules! Depcrate_hmacimpl_37 {
() => {
// Module: crate::hmac
// Provides: {"impl_37"}
// Dependencies: {}
impl HmacRef { pub fn update (& mut self , data : & [u8]) -> OpenSSLResult < () > { unsafe { cvt (ffi :: HMAC_Update (self . as_ptr () , data . as_ptr () , data . len ())) ? ; } Ok (()) } pub fn finish (& mut self) -> OpenSSLResult < DigestBytes > { let mut buf = [0 ; ffi :: EVP_MAX_MD_SIZE as usize] ; let mut len = ffi :: EVP_MAX_MD_SIZE as std :: os :: raw :: c_uint ; unsafe { cvt (ffi :: HMAC_Final (self . as_ptr () , buf . as_mut_ptr () , & mut len)) ? ; } Ok (DigestBytes { buf , len : len . try_into () . unwrap () , }) } pub fn copy (& self) -> OpenSSLResult < Hmac > { unsafe { let h = Hmac :: from_ptr (cvt_p (ffi :: HMAC_CTX_new ()) ?) ; cvt (ffi :: HMAC_CTX_copy (h . as_ptr () , self . as_ptr ())) ? ; Ok (h) } } }
};
}
