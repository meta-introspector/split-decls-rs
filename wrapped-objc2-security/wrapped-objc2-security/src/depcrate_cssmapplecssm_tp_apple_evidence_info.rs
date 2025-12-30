// Generated macro for CSSM_TP_APPLE_EVIDENCE_INFO (struct)
macro_rules! Depcrate_cssmappleCSSM_TP_APPLE_EVIDENCE_INFO {
() => {
// Module: crate::cssmapple
// Provides: {"CSSM_TP_APPLE_EVIDENCE_INFO"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/security/cssm_tp_apple_evidence_info?language=objc)"] # [cfg (all (feature = "SecAsn1Types" , feature = "cssmconfig" , feature = "cssmtype"))] # [deprecated] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq)] pub struct CSSM_TP_APPLE_EVIDENCE_INFO { pub StatusBits : CSSM_TP_APPLE_CERT_STATUS , pub NumStatusCodes : uint32 , pub StatusCodes : * mut CSSM_RETURN , pub Index : uint32 , pub DlDbHandle : CSSM_DL_DB_HANDLE , pub UniqueRecord : CSSM_DB_UNIQUE_RECORD_PTR , # [cfg (all (target_vendor = "apple" , not (target_os = "macos")))] pub CrlReason : sint32 , }
};
}
