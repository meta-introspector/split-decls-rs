// Generated macro for macro_2477 (macro)
macro_rules! Depcrate_ntrtlmacro_2477 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2477"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlQueryPackageClaims (TokenHandle : HANDLE , PackageFullName : PWSTR , PackageSize : PSIZE_T , AppId : PWSTR , AppIdSize : PSIZE_T , DynamicId : * mut GUID , PkgClaim : PPS_PKG_CLAIM , AttributesPresent : PULONG64 ,) -> NTSTATUS ; fn RtlQueryProtectedPolicy (PolicyGuid : * mut GUID , PolicyValue : PULONG_PTR ,) -> NTSTATUS ; fn RtlSetProtectedPolicy (PolicyGuid : * mut GUID , PolicyValue : ULONG_PTR , OldPolicyValue : PULONG_PTR ,) -> NTSTATUS ; fn RtlIsMultiSessionSku () -> BOOLEAN ; fn RtlIsMultiUsersInSessionSku () -> BOOLEAN ; } }
};
}
