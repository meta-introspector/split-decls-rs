// Generated macro for macro_1001 (macro)
macro_rules! Depcrate_ntioapimacro_1001 {
() => {
// Module: crate::ntioapi
// Provides: {"macro_1001"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtNotifyChangeSession (SessionHandle : HANDLE , ChangeSequenceNumber : ULONG , ChangeTimeStamp : PLARGE_INTEGER , Event : IO_SESSION_EVENT , NewState : IO_SESSION_STATE , PreviousState : IO_SESSION_STATE , Payload : PVOID , PayloadSize : ULONG ,) -> NTSTATUS ; } }
};
}
