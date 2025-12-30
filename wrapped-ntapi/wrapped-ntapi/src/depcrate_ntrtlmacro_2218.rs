// Generated macro for macro_2218 (macro)
macro_rules! Depcrate_ntrtlmacro_2218 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2218"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlRemoteCall (Process : HANDLE , Thread : HANDLE , CallSite : PVOID , ArgumentCount : ULONG , Arguments : PULONG_PTR , PassContext : BOOLEAN , AlreadySuspended : BOOLEAN ,) -> NTSTATUS ; fn RtlAddVectoredExceptionHandler (First : ULONG , Handler : PVECTORED_EXCEPTION_HANDLER ,) -> PVOID ; fn RtlRemoveVectoredExceptionHandler (Handle : PVOID ,) -> ULONG ; fn RtlAddVectoredContinueHandler (First : ULONG , Handler : PVECTORED_EXCEPTION_HANDLER ,) -> PVOID ; fn RtlRemoveVectoredContinueHandler (Handle : PVOID ,) -> ULONG ; } }
};
}
