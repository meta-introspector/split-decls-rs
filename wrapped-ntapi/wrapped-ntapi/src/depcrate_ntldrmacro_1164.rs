// Generated macro for macro_1164 (macro)
macro_rules! Depcrate_ntldrmacro_1164 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1164"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrSystemDllInitBlock () -> PPS_SYSTEM_DLL_INIT_BLOCK ; fn LdrAddLoadAsDataTable (Module : PVOID , FilePath : PWSTR , Size : SIZE_T , Handle : HANDLE ,) -> NTSTATUS ; fn LdrRemoveLoadAsDataTable (InitModule : PVOID , BaseModule : * mut PVOID , Size : PSIZE_T , Flags : ULONG ,) -> NTSTATUS ; fn LdrGetFileNameFromLoadAsDataTable (Module : PVOID , pFileNamePrt : * mut PVOID ,) -> NTSTATUS ; fn LdrDisableThreadCalloutsForDll (DllImageBase : PVOID ,) -> NTSTATUS ; fn LdrAccessResource (DllHandle : PVOID , ResourceDataEntry : PIMAGE_RESOURCE_DATA_ENTRY , ResourceBuffer : * mut PVOID , ResourceLength : * mut ULONG ,) -> NTSTATUS ; } }
};
}
