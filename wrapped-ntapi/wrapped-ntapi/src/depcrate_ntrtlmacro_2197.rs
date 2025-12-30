// Generated macro for macro_2197 (macro)
macro_rules! Depcrate_ntrtlmacro_2197 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2197"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCloneUserProcess (ProcessFlags : ULONG , ProcessSecurityDescriptor : PSECURITY_DESCRIPTOR , ThreadSecurityDescriptor : PSECURITY_DESCRIPTOR , DebugPort : HANDLE , ProcessInformation : PRTL_USER_PROCESS_INFORMATION ,) -> NTSTATUS ; fn RtlUpdateClonedCriticalSection (CriticalSection : PRTL_CRITICAL_SECTION ,) ; fn RtlUpdateClonedSRWLock (SRWLock : PRTL_SRWLOCK , Shared : LOGICAL ,) ; } }
};
}
