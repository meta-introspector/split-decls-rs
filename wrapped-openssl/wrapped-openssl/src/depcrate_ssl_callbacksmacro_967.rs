// Generated macro for macro_967 (macro)
macro_rules! Depcrate_ssl_callbacksmacro_967 {
() => {
// Module: crate::ssl::callbacks
// Provides: {"macro_967"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl , boringssl , awslc))] { type DataPtr = * const c_uchar ; } else { type DataPtr = * mut c_uchar ; } }
};
}
