// Generated macro for macro_141 (macro)
macro_rules! Depcrate_ntexapimacro_141 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_141"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtCreateTimer (TimerHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES , TimerType : TIMER_TYPE ,) -> NTSTATUS ; fn NtOpenTimer (TimerHandle : PHANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn NtSetTimer (TimerHandle : HANDLE , DueTime : PLARGE_INTEGER , TimerApcRoutine : PTIMER_APC_ROUTINE , TimerContext : PVOID , ResumeTimer : BOOLEAN , Period : LONG , PreviousState : PBOOLEAN ,) -> NTSTATUS ; fn NtSetTimerEx (TimerHandle : HANDLE , TimerSetInformationClass : TIMER_SET_INFORMATION_CLASS , TimerSetInformation : PVOID , TimerSetInformationLength : ULONG ,) -> NTSTATUS ; fn NtCancelTimer (TimerHandle : HANDLE , CurrentState : PBOOLEAN ,) -> NTSTATUS ; fn NtQueryTimer (TimerHandle : HANDLE , TimerInformationClass : TIMER_INFORMATION_CLASS , TimerInformation : PVOID , TimerInformationLength : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; fn NtCreateIRTimer (TimerHandle : PHANDLE , DesiredAccess : ACCESS_MASK ,) -> NTSTATUS ; fn NtSetIRTimer (TimerHandle : HANDLE , DueTime : PLARGE_INTEGER ,) -> NTSTATUS ; } }
};
}
