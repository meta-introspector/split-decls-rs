// Generated macro for macro_1876 (macro)
macro_rules! Depcrate_ntpsapimacro_1876 {
() => {
// Module: crate::ntpsapi
// Provides: {"macro_1876"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtQueueApcThreadEx (ThreadHandle : HANDLE , UserApcReserveHandle : HANDLE , ApcRoutine : PPS_APC_ROUTINE , ApcArgument1 : PVOID , ApcArgument2 : PVOID , ApcArgument3 : PVOID ,) -> NTSTATUS ; fn NtAlertThreadByThreadId (ThreadId : HANDLE ,) -> NTSTATUS ; fn NtWaitForAlertByThreadId (Address : PVOID , Timeout : PLARGE_INTEGER ,) -> NTSTATUS ; } }
};
}
