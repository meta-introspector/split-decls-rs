// Generated macro for macro_127 (macro)
macro_rules! Depcrate_asn1macro_127 {
() => {
// Module: crate::asn1
// Provides: {"macro_127"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl , boringssl , awslc))] { use ffi :: ASN1_STRING_get0_data ; } else { # [allow (bad_style)] unsafe fn ASN1_STRING_get0_data (s : * mut ffi :: ASN1_STRING) -> * const :: libc :: c_uchar { ffi :: ASN1_STRING_data (s) } } }
};
}
