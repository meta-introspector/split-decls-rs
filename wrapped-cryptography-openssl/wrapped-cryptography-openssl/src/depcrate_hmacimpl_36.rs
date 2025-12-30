// Generated macro for impl_36 (impl)
macro_rules! Depcrate_hmacimpl_36 {
() => {
// Module: crate::hmac
// Provides: {"impl_36"}
// Dependencies: {}
impl Hmac { # [cfg_attr (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC) , allow (clippy :: useless_conversion))] pub fn new (key : & [u8] , md : openssl :: hash :: MessageDigest) -> OpenSSLResult < Hmac > { unsafe { let h = Hmac :: from_ptr (cvt_p (ffi :: HMAC_CTX_new ()) ?) ; cvt (ffi :: HMAC_Init_ex (h . as_ptr () , key . as_ptr () . cast () , key . len () . try_into () . expect ("Key too long for OpenSSL's length type") , md . as_ptr () , ptr :: null_mut () ,)) ? ; Ok (h) } } }
};
}
