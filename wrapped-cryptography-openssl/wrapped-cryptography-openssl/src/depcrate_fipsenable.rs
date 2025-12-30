// Generated macro for enable (function)
macro_rules! Depcrate_fipsenable {
() => {
// Module: crate::fips
// Provides: {"enable"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] pub fn enable () -> OpenSSLResult < () > { unsafe { cvt (ffi :: EVP_default_properties_enable_fips (ptr :: null_mut () , 1)) ? ; } Ok (()) }
};
}
