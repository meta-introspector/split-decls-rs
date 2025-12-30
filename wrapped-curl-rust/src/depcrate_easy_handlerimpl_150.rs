// Generated macro for impl_150 (impl)
macro_rules! Depcrate_easy_handlerimpl_150 {
() => {
// Module: crate::easy::handler
// Provides: {"impl_150"}
// Dependencies: {}
impl fmt :: Debug for SslOpt { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("SslOpt") . field ("no_revoke" , & (self . bits & curl_sys :: CURLSSLOPT_NO_REVOKE != 0) ,) . field ("allow_beast" , & (self . bits & curl_sys :: CURLSSLOPT_ALLOW_BEAST != 0) ,) . finish () } }
};
}
