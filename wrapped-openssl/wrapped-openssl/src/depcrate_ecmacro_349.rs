// Generated macro for macro_349 (macro)
macro_rules! Depcrate_ecmacro_349 {
() => {
// Module: crate::ec
// Provides: {"macro_349"}
// Dependencies: {}
generic_foreign_type_and_impl_send_sync ! { type CType = ffi :: EC_KEY ; fn drop = ffi :: EC_KEY_free ; # [doc = " Public and optional private key on the given curve."] pub struct EcKey < T >; # [doc = " A reference to an [`EcKey`]."] pub struct EcKeyRef < T >; }
};
}
