// Generated macro for macro_152 (macro)
macro_rules! Depcrate_ntexapimacro_152 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_152"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateKeyedEvent (KeyedEventHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , Flags : ULONG ,) -> NTSTATUS ; fn NtOpenKeyedEvent (KeyedEventHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtReleaseKeyedEvent (KeyedEventHandle : HANDLE , KeyValue : PVOID , Alertable : BOOLEAN , Timeout : PLARGE_INTEGER ,) -> NTSTATUS ; fn NtWaitForKeyedEvent (KeyedEventHandle : HANDLE , KeyValue : PVOID , Alertable : BOOLEAN , Timeout : PLARGE_INTEGER ,) -> NTSTATUS ; fn NtUmsThreadYield (SchedulerParam : PVOID ,) -> NTSTATUS ; } }
};
}
