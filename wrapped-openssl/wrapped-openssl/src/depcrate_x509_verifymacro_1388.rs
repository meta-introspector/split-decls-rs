// Generated macro for macro_1388 (macro)
macro_rules! Depcrate_x509_verifymacro_1388 {
() => {
// Module: crate::x509::verify
// Provides: {"macro_1388"}
// Dependencies: {}
bitflags ! { # [doc = " Flags used to check an `X509` certificate."] # [derive (Copy , Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct X509CheckFlags : c_uint { const ALWAYS_CHECK_SUBJECT = ffi :: X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT as _ ; const NO_WILDCARDS = ffi :: X509_CHECK_FLAG_NO_WILDCARDS as _ ; const NO_PARTIAL_WILDCARDS = ffi :: X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS as _ ; const MULTI_LABEL_WILDCARDS = ffi :: X509_CHECK_FLAG_MULTI_LABEL_WILDCARDS as _ ; const SINGLE_LABEL_SUBDOMAINS = ffi :: X509_CHECK_FLAG_SINGLE_LABEL_SUBDOMAINS as _ ; # [doc = " Requires OpenSSL 1.1.0 or newer."] # [cfg (any (ossl110))] const NEVER_CHECK_SUBJECT = ffi :: X509_CHECK_FLAG_NEVER_CHECK_SUBJECT ; # [deprecated (since = "0.10.6" , note = "renamed to NO_WILDCARDS")] const FLAG_NO_WILDCARDS = ffi :: X509_CHECK_FLAG_NO_WILDCARDS as _ ; } }
};
}
