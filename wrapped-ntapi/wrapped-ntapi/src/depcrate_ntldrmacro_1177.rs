// Generated macro for macro_1177 (macro)
macro_rules! Depcrate_ntldrmacro_1177 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1177"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrEnumResources (DllHandle : PVOID , ResourceInfo : PLDR_RESOURCE_INFO , Level : ULONG , ResourceCount : * mut ULONG , Resources : PLDR_ENUM_RESOURCE_ENTRY ,) -> NTSTATUS ; fn LdrFindEntryForAddress (DllHandle : PVOID , Entry : * mut PLDR_DATA_TABLE_ENTRY ,) -> NTSTATUS ; } }
};
}
