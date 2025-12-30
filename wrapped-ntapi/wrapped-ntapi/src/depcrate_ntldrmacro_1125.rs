// Generated macro for macro_1125 (macro)
macro_rules! Depcrate_ntldrmacro_1125 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1125"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrGetDllHandleEx (Flags : ULONG , DllPath : PWSTR , DllCharacteristics : PULONG , DllName : PUNICODE_STRING , DllHandle : * mut PVOID ,) -> NTSTATUS ; fn LdrGetDllHandleByMapping (BaseAddress : PVOID , DllHandle : * mut PVOID ,) -> NTSTATUS ; fn LdrGetDllHandleByName (BaseDllName : PUNICODE_STRING , FullDllName : PUNICODE_STRING , DllHandle : * mut PVOID ,) -> NTSTATUS ; fn LdrGetDllFullName (DllHandle : PVOID , FullDllName : PUNICODE_STRING ,) -> NTSTATUS ; fn LdrGetDllDirectory (DllDirectory : PUNICODE_STRING ,) -> NTSTATUS ; fn LdrSetDllDirectory (DllDirectory : PUNICODE_STRING ,) -> NTSTATUS ; } }
};
}
