// Generated macro for is_enabled (function)
macro_rules! Depcrate_fipsis_enabled {
() => {
// Module: crate::fips
// Provides: {"is_enabled"}
// Dependencies: {}
pub fn is_enabled () -> bool { cfg_if :: cfg_if ! { if # [cfg (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL))] { false } else if # [cfg (CRYPTOGRAPHY_IS_AWSLC)] { openssl :: fips :: enabled () } else { unsafe { ffi :: EVP_default_properties_is_fips_enabled (ptr :: null_mut ()) == 1 } } } }
};
}
