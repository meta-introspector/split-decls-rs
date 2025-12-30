// Generated macro for impl_1600 (impl)
macro_rules! Depcrate_x509impl_1600 {
() => {
// Module: crate::x509
// Provides: {"impl_1600"}
// Dependencies: {}
impl X509VerifyResult { # [doc = " Creates an `X509VerifyResult` from a raw error number."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Some methods on `X509VerifyResult` are not thread safe if the error"] # [doc = " number is invalid."] pub unsafe fn from_raw (err : c_int) -> X509VerifyResult { X509VerifyResult (err) } # [doc = " Return the integer representation of an `X509VerifyResult`."] # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } # [doc = " Return a human readable error string from the verification error."] # [corresponds (X509_verify_cert_error_string)] # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn error_string (& self) -> & 'static str { ffi :: init () ; unsafe { let s = ffi :: X509_verify_cert_error_string (self . 0 as c_long) ; str :: from_utf8 (CStr :: from_ptr (s) . to_bytes ()) . unwrap () } } # [doc = " Successful peer certificate verification."] pub const OK : X509VerifyResult = X509VerifyResult (ffi :: X509_V_OK) ; # [doc = " Application verification failure."] pub const APPLICATION_VERIFICATION : X509VerifyResult = X509VerifyResult (ffi :: X509_V_ERR_APPLICATION_VERIFICATION) ; }
};
}
