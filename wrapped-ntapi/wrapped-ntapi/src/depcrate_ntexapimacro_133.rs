// Generated macro for macro_133 (macro)
macro_rules! Depcrate_ntexapimacro_133 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_133"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateSemaphore (SemaphoreHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , InitialCount : LONG , MaximumCount : LONG ,) -> NTSTATUS ; fn NtOpenSemaphore (SemaphoreHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtReleaseSemaphore (SemaphoreHandle : HANDLE , ReleaseCount : LONG , PreviousCount : PLONG ,) -> NTSTATUS ; fn NtQuerySemaphore (SemaphoreHandle : HANDLE , SemaphoreInformationClass : SEMAPHORE_INFORMATION_CLASS , SemaphoreInformation : PVOID , SemaphoreInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; } }
};
}
