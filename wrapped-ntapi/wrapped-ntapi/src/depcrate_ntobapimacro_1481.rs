// Generated macro for macro_1481 (macro)
macro_rules! Depcrate_ntobapimacro_1481 {
() => {
// Module: crate::ntobapi
// Provides: {"macro_1481"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtQueryObject (Handle : HANDLE , ObjectInformationClass : OBJECT_INFORMATION_CLASS , ObjectInformation : PVOID , ObjectInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; fn NtSetInformationObject (Handle : HANDLE , ObjectInformationClass : OBJECT_INFORMATION_CLASS , ObjectInformation : PVOID , ObjectInformationLength : ULONG ,) -> NTSTATUS ; } }
};
}
