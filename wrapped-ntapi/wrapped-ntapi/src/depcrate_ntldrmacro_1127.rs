// Generated macro for macro_1127 (macro)
macro_rules! Depcrate_ntldrmacro_1127 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1127"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrAddRefDll (Flags : ULONG , DllHandle : PVOID ,) -> NTSTATUS ; fn LdrGetProcedureAddress (DllHandle : PVOID , ProcedureName : PANSI_STRING , ProcedureNumber : ULONG , ProcedureAddress : * mut PVOID ,) -> NTSTATUS ; } }
};
}
