// Generated macro for macro_1438 (macro)
macro_rules! Depcrate_x509_storemacro_1438 {
() => {
// Module: crate::x509::store
// Provides: {"macro_1438"}
// Dependencies: {}
generic_foreign_type_and_impl_send_sync ! { type CType = ffi :: X509_LOOKUP ; fn drop = ffi :: X509_LOOKUP_free ; # [doc = " Information used by an `X509Store` to look up certificates and CRLs."] pub struct X509Lookup < T >; # [doc = " A reference to an [`X509Lookup`]."] pub struct X509LookupRef < T >; }
};
}
