// Generated macro for macro_973 (macro)
macro_rules! Depcrate_ssl_callbacksmacro_973 {
() => {
// Module: crate::ssl::callbacks
// Provides: {"macro_973"}
// Dependencies: {}
# [cfg (not (any (boringssl , awslc)))] cfg_if ! { if # [cfg (any (ossl110 , libressl))] { type CookiePtr = * const c_uchar ; } else { type CookiePtr = * mut c_uchar ; } }
};
}
