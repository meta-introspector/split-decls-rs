// Generated macro for macro_2132 (macro)
macro_rules! Depcrate_ntrtlmacro_2132 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2132"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlInitializeResource (Resource : PRTL_RESOURCE ,) ; fn RtlDeleteResource (Resource : PRTL_RESOURCE ,) ; fn RtlAcquireResourceShared (Resource : PRTL_RESOURCE , Wait : BOOLEAN ,) -> BOOLEAN ; fn RtlAcquireResourceExclusive (Resource : PRTL_RESOURCE , Wait : BOOLEAN ,) -> BOOLEAN ; fn RtlReleaseResource (Resource : PRTL_RESOURCE ,) ; fn RtlConvertSharedToExclusive (Resource : PRTL_RESOURCE ,) ; fn RtlConvertExclusiveToShared (Resource : PRTL_RESOURCE ,) ; fn RtlInitializeSRWLock (SRWLock : PRTL_SRWLOCK ,) ; fn RtlAcquireSRWLockExclusive (SRWLock : PRTL_SRWLOCK ,) ; fn RtlAcquireSRWLockShared (SRWLock : PRTL_SRWLOCK ,) ; fn RtlReleaseSRWLockExclusive (SRWLock : PRTL_SRWLOCK ,) ; fn RtlReleaseSRWLockShared (SRWLock : PRTL_SRWLOCK ,) ; fn RtlTryAcquireSRWLockExclusive (SRWLock : PRTL_SRWLOCK ,) -> BOOLEAN ; fn RtlTryAcquireSRWLockShared (SRWLock : PRTL_SRWLOCK ,) -> BOOLEAN ; fn RtlAcquireReleaseSRWLockExclusive (SRWLock : PRTL_SRWLOCK ,) ; fn RtlInitializeConditionVariable (ConditionVariable : PRTL_CONDITION_VARIABLE ,) ; fn RtlSleepConditionVariableCS (ConditionVariable : PRTL_CONDITION_VARIABLE , CriticalSection : PRTL_CRITICAL_SECTION , Timeout : PLARGE_INTEGER ,) -> NTSTATUS ; fn RtlSleepConditionVariableSRW (ConditionVariable : PRTL_CONDITION_VARIABLE , SRWLock : PRTL_SRWLOCK , Timeout : PLARGE_INTEGER , Flags : ULONG ,) -> NTSTATUS ; fn RtlWakeConditionVariable (ConditionVariable : PRTL_CONDITION_VARIABLE ,) ; fn RtlWakeAllConditionVariable (ConditionVariable : PRTL_CONDITION_VARIABLE ,) ; } }
};
}
