// Generated macro for macro_2387 (macro)
macro_rules! Depcrate_ntrtlmacro_2387 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2387"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlIdentifierAuthoritySid (Sid : PSID ,) -> PSID_IDENTIFIER_AUTHORITY ; fn RtlSubAuthoritySid (Sid : PSID , SubAuthority : ULONG ,) -> PULONG ; fn RtlSubAuthorityCountSid (Sid : PSID ,) -> PUCHAR ; fn RtlLengthSid (Sid : PSID ,) -> ULONG ; fn RtlCopySid (DestinationSidLength : ULONG , DestinationSid : PSID , SourceSid : PSID ,) -> NTSTATUS ; fn RtlCopySidAndAttributesArray (Count : ULONG , Src : PSID_AND_ATTRIBUTES , SidAreaSize : ULONG , Dest : PSID_AND_ATTRIBUTES , SidArea : PSID , RemainingSidArea : * mut PSID , RemainingSidAreaSize : PULONG ,) -> NTSTATUS ; fn RtlCreateServiceSid (ServiceName : PUNICODE_STRING , ServiceSid : PSID , ServiceSidLength : PULONG ,) -> NTSTATUS ; fn RtlSidDominates (Sid1 : PSID , Sid2 : PSID , Dominates : PBOOLEAN ,) -> NTSTATUS ; fn RtlSidDominatesForTrust (Sid1 : PSID , Sid2 : PSID , DominatesTrust : PBOOLEAN ,) -> NTSTATUS ; fn RtlSidEqualLevel (Sid1 : PSID , Sid2 : PSID , EqualLevel : PBOOLEAN ,) -> NTSTATUS ; fn RtlSidIsHigherLevel (Sid1 : PSID , Sid2 : PSID , HigherLevel : PBOOLEAN ,) -> NTSTATUS ; fn RtlCreateVirtualAccountSid (Name : PCUNICODE_STRING , BaseSubAuthority : ULONG , Sid : PSID , SidLength : PULONG ,) -> NTSTATUS ; fn RtlReplaceSidInSd (SecurityDescriptor : PSECURITY_DESCRIPTOR , OldSid : PSID , NewSid : PSID , NumChanges : * mut ULONG ,) -> NTSTATUS ; } }
};
}
