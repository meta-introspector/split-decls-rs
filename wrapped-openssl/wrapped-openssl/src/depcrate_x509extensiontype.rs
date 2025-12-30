// Generated macro for ExtensionType (trait)
macro_rules! Depcrate_x509ExtensionType {
() => {
// Module: crate::x509
// Provides: {"ExtensionType"}
// Dependencies: {}
# [doc = " A type of X509 extension."] # [doc = ""] # [doc = " # Safety"] # [doc = " The value of NID and Output must match those in OpenSSL so that"] # [doc = " `Output::from_ptr_opt(*_get_ext_d2i(*, NID, ...))` is valid."] pub unsafe trait ExtensionType { const NID : Nid ; type Output : ForeignType ; }
};
}
