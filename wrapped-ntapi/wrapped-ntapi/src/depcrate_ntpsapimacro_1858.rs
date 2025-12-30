// Generated macro for macro_1858 (macro)
macro_rules! Depcrate_ntpsapimacro_1858 {
() => {
// Module: crate::ntpsapi
// Provides: {"macro_1858"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateProcessEx (ProcessHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , ParentProcess : HANDLE , Flags : ULONG , SectionHandle : HANDLE , DebugPort : HANDLE , ExceptionPort : HANDLE , JobMemberLevel : ULONG ,) -> NTSTATUS ; fn NtOpenProcess (ProcessHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , ClientId : PCLIENT_ID ,) -> NTSTATUS ; fn NtTerminateProcess (ProcessHandle : HANDLE , ExitStatus : NTSTATUS ,) -> NTSTATUS ; fn NtSuspendProcess (ProcessHandle : HANDLE ,) -> NTSTATUS ; fn NtResumeProcess (ProcessHandle : HANDLE ,) -> NTSTATUS ; } }
};
}
