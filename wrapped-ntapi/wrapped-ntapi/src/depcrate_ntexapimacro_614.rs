// Generated macro for macro_614 (macro)
macro_rules! Depcrate_ntexapimacro_614 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_614"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtQueryLicenseValue (ValueName : PUNICODE_STRING , Type : PULONG , Data : PVOID , DataSize : ULONG , ResultDataSize : PULONG ,) -> NTSTATUS ; fn NtSetDefaultHardErrorPort (DefaultHardErrorPort : HANDLE ,) -> NTSTATUS ; } }
};
}
