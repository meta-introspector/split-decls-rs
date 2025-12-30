// Generated macro for macro_2204 (macro)
macro_rules! Depcrate_ntrtlmacro_2204 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2204"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateUserThread (Process : HANDLE , ThreadSecurityDescriptor : PSECURITY_DESCRIPTOR , CreateSuspended : BOOLEAN , ZeroBits : ULONG , MaximumStackSize : SIZE_T , CommittedStackSize : SIZE_T , StartAddress : PUSER_THREAD_START_ROUTINE , Parameter : PVOID , Thread : PHANDLE , ClientId : PCLIENT_ID ,) -> NTSTATUS ; fn RtlExitUserThread (ExitStatus : NTSTATUS ,) ; fn RtlIsCurrentThreadAttachExempt () -> BOOLEAN ; fn RtlCreateUserStack (CommittedStackSize : SIZE_T , MaximumStackSize : SIZE_T , ZeroBits : ULONG_PTR , PageSize : SIZE_T , ReserveAlignment : ULONG_PTR , InitialTeb : PINITIAL_TEB ,) -> NTSTATUS ; fn RtlFreeUserStack (AllocationBase : PVOID ,) -> NTSTATUS ; } }
};
}
