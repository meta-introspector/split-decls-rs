// Generated macro for macro_2395 (macro)
macro_rules! Depcrate_ntrtlmacro_2395 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2395"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlSetThreadPoolStartFunc (StartPoolThread : PRTL_START_POOL_THREAD , ExitPoolThread : PRTL_EXIT_POOL_THREAD ,) -> NTSTATUS ; fn RtlUserThreadStart (Function : PTHREAD_START_ROUTINE , Parameter : PVOID ,) ; fn LdrInitializeThunk (ContextRecord : PCONTEXT , Parameter : PVOID ,) ; fn RtlCreateTimerQueue (TimerQueueHandle : PHANDLE ,) -> NTSTATUS ; fn RtlCreateTimer (TimerQueueHandle : HANDLE , Handle : PHANDLE , Function : WAITORTIMERCALLBACKFUNC , Context : PVOID , DueTime : ULONG , Period : ULONG , Flags : ULONG ,) -> NTSTATUS ; fn RtlUpdateTimer (TimerQueueHandle : HANDLE , TimerHandle : HANDLE , DueTime : ULONG , Period : ULONG ,) -> NTSTATUS ; fn RtlDeleteTimer (TimerQueueHandle : HANDLE , TimerToCancel : HANDLE , Event : HANDLE ,) -> NTSTATUS ; fn RtlDeleteTimerQueue (TimerQueueHandle : HANDLE ,) -> NTSTATUS ; fn RtlDeleteTimerQueueEx (TimerQueueHandle : HANDLE , Event : HANDLE ,) -> NTSTATUS ; fn RtlFormatCurrentUserKeyPath (CurrentUserKeyPath : PUNICODE_STRING ,) -> NTSTATUS ; fn RtlOpenCurrentUser (DesiredAccess : ACCESS_MASK , CurrentUserKey : PHANDLE ,) -> NTSTATUS ; } }
};
}
