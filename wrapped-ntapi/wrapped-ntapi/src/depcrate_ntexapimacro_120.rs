// Generated macro for macro_120 (macro)
macro_rules! Depcrate_ntexapimacro_120 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_120"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateEvent (EventHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , EventType : EVENT_TYPE , InitialState : BOOLEAN ,) -> NTSTATUS ; fn NtOpenEvent (EventHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtSetEvent (EventHandle : HANDLE , PreviousState : PLONG ,) -> NTSTATUS ; fn NtSetEventBoostPriority (EventHandle : HANDLE ,) -> NTSTATUS ; fn NtClearEvent (EventHandle : HANDLE ,) -> NTSTATUS ; fn NtResetEvent (EventHandle : HANDLE , PreviousState : PLONG ,) -> NTSTATUS ; fn NtPulseEvent (EventHandle : HANDLE , PreviousState : PLONG ,) -> NTSTATUS ; fn NtQueryEvent (EventHandle : HANDLE , EventInformationClass : EVENT_INFORMATION_CLASS , EventInformation : PVOID , EventInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; } }
};
}
