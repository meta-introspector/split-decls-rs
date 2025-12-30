// Generated macro for macro_523 (macro)
macro_rules! Depcrate_ntexapimacro_523 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_523"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtQuerySystemInformation (SystemInformationClass : SYSTEM_INFORMATION_CLASS , SystemInformation : PVOID , SystemInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; fn NtQuerySystemInformationEx (SystemInformationClass : SYSTEM_INFORMATION_CLASS , InputBuffer : PVOID , InputBufferLength : ULONG , SystemInformation : PVOID , SystemInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; fn NtSetSystemInformation (SystemInformationClass : SYSTEM_INFORMATION_CLASS , SystemInformation : PVOID , SystemInformationLength : ULONG ,) -> NTSTATUS ; } }
};
}
