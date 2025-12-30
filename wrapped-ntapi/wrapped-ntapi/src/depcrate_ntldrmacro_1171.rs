// Generated macro for macro_1171 (macro)
macro_rules! Depcrate_ntldrmacro_1171 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1171"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrFindResource_U (DllHandle : PVOID , ResourceInfo : PLDR_RESOURCE_INFO , Level : ULONG , ResourceDataEntry : * mut PIMAGE_RESOURCE_DATA_ENTRY ,) -> NTSTATUS ; fn LdrFindResourceDirectory_U (DllHandle : PVOID , ResourceInfo : PLDR_RESOURCE_INFO , Level : ULONG , ResourceDirectory : * mut PIMAGE_RESOURCE_DIRECTORY ,) -> NTSTATUS ; } }
};
}
