// Generated macro for impl_5156 (impl)
macro_rules! Depcrate_cssmappleimpl_5156 {
() => {
// Module: crate::cssmapple
// Provides: {"impl_5156"}
// Dependencies: {}
# [cfg (all (feature = "SecAsn1Types" , feature = "cssmconfig" , feature = "cssmtype" , feature = "objc2"))] unsafe impl Encode for CSSM_TP_APPLE_EVIDENCE_INFO { const ENCODING : Encoding = Encoding :: Struct ("?" , & [< CSSM_TP_APPLE_CERT_STATUS > :: ENCODING , < uint32 > :: ENCODING , < * mut CSSM_RETURN > :: ENCODING , < uint32 > :: ENCODING , < CSSM_DL_DB_HANDLE > :: ENCODING , < CSSM_DB_UNIQUE_RECORD_PTR > :: ENCODING , # [cfg (all (target_vendor = "apple" , not (target_os = "macos")))] < sint32 > :: ENCODING ,] ,) ; }
};
}
