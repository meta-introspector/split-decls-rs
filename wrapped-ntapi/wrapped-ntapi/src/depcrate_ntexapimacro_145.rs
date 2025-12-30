// Generated macro for macro_145 (macro)
macro_rules! Depcrate_ntexapimacro_145 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_145"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateTimer2 (TimerHandle : PHANDLE , Reserved1 : PVOID , Reserved2 : PVOID , Attributes : ULONG , DesiredAccess : ACCESS_MASK ,) -> NTSTATUS ; fn NtSetTimer2 (TimerHandle : HANDLE , DueTime : PLARGE_INTEGER , Period : PLARGE_INTEGER , Parameters : PT2_SET_PARAMETERS ,) -> NTSTATUS ; fn NtCancelTimer2 (TimerHandle : HANDLE , Parameters : PT2_CANCEL_PARAMETERS ,) -> NTSTATUS ; } }
};
}
