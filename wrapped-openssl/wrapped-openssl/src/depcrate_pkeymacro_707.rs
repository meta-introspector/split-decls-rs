// Generated macro for macro_707 (macro)
macro_rules! Depcrate_pkeymacro_707 {
() => {
// Module: crate::pkey
// Provides: {"macro_707"}
// Dependencies: {}
generic_foreign_type_and_impl_send_sync ! { type CType = ffi :: EVP_PKEY ; fn drop = ffi :: EVP_PKEY_free ; # [doc = " A public or private key."] pub struct PKey < T >; # [doc = " Reference to `PKey`."] pub struct PKeyRef < T >; }
};
}
