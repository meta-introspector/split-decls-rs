// Generated macro for macro_1122 (macro)
macro_rules! Depcrate_ntldrmacro_1122 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1122"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrLoadDll (DllPath : PWSTR , DllCharacteristics : PULONG , DllName : PUNICODE_STRING , DllHandle : * mut PVOID ,) -> NTSTATUS ; fn LdrUnloadDll (DllHandle : PVOID ,) -> NTSTATUS ; fn LdrGetDllHandle (DllPath : PWSTR , DllCharacteristics : PULONG , DllName : PUNICODE_STRING , DllHandle : * mut PVOID ,) -> NTSTATUS ; } }
};
}
