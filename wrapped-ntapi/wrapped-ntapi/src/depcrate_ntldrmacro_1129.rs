// Generated macro for macro_1129 (macro)
macro_rules! Depcrate_ntldrmacro_1129 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1129"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrGetProcedureAddressEx (DllHandle : PVOID , ProcedureName : PANSI_STRING , ProcedureNumber : ULONG , ProcedureAddress : * mut PVOID , Flags : ULONG ,) -> NTSTATUS ; fn LdrGetKnownDllSectionHandle (DllName : PCWSTR , KnownDlls32 : BOOLEAN , Section : PHANDLE ,) -> NTSTATUS ; fn LdrGetProcedureAddressForCaller (DllHandle : PVOID , ProcedureName : PANSI_STRING , ProcedureNumber : ULONG , ProcedureAddress : * mut PVOID , Flags : ULONG , Callback : * mut PVOID ,) -> NTSTATUS ; } }
};
}
