// Generated macro for macro_1971 (macro)
macro_rules! Depcrate_ntpsapimacro_1971 {
() => {
// Module: crate::ntpsapi
// Provides: {"macro_1971"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateJobObject (JobHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtOpenJobObject (JobHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtAssignProcessToJobObject (JobHandle : HANDLE , ProcessHandle : HANDLE ,) -> NTSTATUS ; fn NtTerminateJobObject (JobHandle : HANDLE , ExitStatus : NTSTATUS ,) -> NTSTATUS ; fn NtIsProcessInJob (ProcessHandle : HANDLE , JobHandle : HANDLE ,) -> NTSTATUS ; fn NtQueryInformationJobObject (JobHandle : HANDLE , JobObjectInformationClass : JOBOBJECTINFOCLASS , JobObjectInformation : PVOID , JobObjectInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; fn NtSetInformationJobObject (JobHandle : HANDLE , JobObjectInformationClass : JOBOBJECTINFOCLASS , JobObjectInformation : PVOID , JobObjectInformationLength : ULONG ,) -> NTSTATUS ; fn NtCreateJobSet (NumJob : ULONG , UserJobSet : PJOB_SET_ARRAY , Flags : ULONG ,) -> NTSTATUS ; fn NtRevertContainerImpersonation () -> NTSTATUS ; } }
};
}
