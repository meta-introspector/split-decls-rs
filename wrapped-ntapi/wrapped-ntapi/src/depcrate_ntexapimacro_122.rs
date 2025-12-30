// Generated macro for macro_122 (macro)
macro_rules! Depcrate_ntexapimacro_122 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_122"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateEventPair (EventPairHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtOpenEventPair (EventPairHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtSetLowEventPair (EventPairHandle : HANDLE ,) -> NTSTATUS ; fn NtSetHighEventPair (EventPairHandle : HANDLE ,) -> NTSTATUS ; fn NtWaitLowEventPair (EventPairHandle : HANDLE ,) -> NTSTATUS ; fn NtWaitHighEventPair (EventPairHandle : HANDLE ,) -> NTSTATUS ; fn NtSetLowWaitHighEventPair (EventPairHandle : HANDLE ,) -> NTSTATUS ; fn NtSetHighWaitLowEventPair (EventPairHandle : HANDLE ,) -> NTSTATUS ; } }
};
}
