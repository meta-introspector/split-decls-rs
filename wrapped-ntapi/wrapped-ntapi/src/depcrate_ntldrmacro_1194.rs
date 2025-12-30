// Generated macro for macro_1194 (macro)
macro_rules! Depcrate_ntldrmacro_1194 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1194"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrResolveDelayLoadedAPI (ParentModuleBase : PVOID , DelayloadDescriptor : PCIMAGE_DELAYLOAD_DESCRIPTOR , FailureDllHook : PDELAYLOAD_FAILURE_DLL_CALLBACK , FailureSystemHook : PDELAYLOAD_FAILURE_SYSTEM_ROUTINE , ThunkAddress : PIMAGE_THUNK_DATA , Flags : ULONG ,) -> PVOID ; fn LdrResolveDelayLoadsFromDll (ParentBase : PVOID , TargetDllName : PCSTR , Flags : ULONG ,) -> NTSTATUS ; fn LdrSetDefaultDllDirectories (DirectoryFlags : ULONG ,) -> NTSTATUS ; fn LdrShutdownProcess () -> NTSTATUS ; fn LdrShutdownThread () -> NTSTATUS ; fn LdrSetImplicitPathOptions (ImplicitPathOptions : ULONG ,) -> NTSTATUS ; fn LdrControlFlowGuardEnforced () -> BOOLEAN ; } }
};
}
