// Generated macro for number (function)
macro_rules! Depcrate_versionnumber {
() => {
// Module: crate::version
// Provides: {"number"}
// Dependencies: {}
# [doc = " OPENSSL_VERSION_NUMBER is a numeric release version identifier:"] # [doc = ""] # [doc = " `MNNFFPPS: major minor fix patch status`"] # [doc = ""] # [doc = " The status nibble has one of the values 0 for development, 1 to e for betas 1 to 14, and f for release."] # [doc = ""] # [doc = " for example"] # [doc = ""] # [doc = " `0x000906000 == 0.9.6 dev`"] # [doc = " `0x000906023 == 0.9.6b beta 3`"] # [doc = " `0x00090605f == 0.9.6e release`"] # [corresponds (OpenSSL_version_num)] pub fn number () -> i64 { unsafe { OpenSSL_version_num () as i64 } }
};
}
