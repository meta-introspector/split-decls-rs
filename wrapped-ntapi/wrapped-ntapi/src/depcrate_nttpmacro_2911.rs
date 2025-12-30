// Generated macro for macro_2911 (macro)
macro_rules! Depcrate_nttpmacro_2911 {
() => {
// Module: crate::nttp
// Provides: {"macro_2911"}
// Dependencies: {}
EXTERN ! { extern "system" { fn TpAllocIoCompletion (IoReturn : * mut PTP_IO , File : HANDLE , Callback : PTP_IO_CALLBACK , Context : PVOID , CallbackEnviron : PTP_CALLBACK_ENVIRON ,) -> NTSTATUS ; fn TpReleaseIoCompletion (Io : PTP_IO ,) ; fn TpStartAsyncIoOperation (Io : PTP_IO ,) ; fn TpCancelAsyncIoOperation (Io : PTP_IO ,) ; fn TpWaitForIoCompletion (Io : PTP_IO , CancelPendingCallbacks : LOGICAL ,) ; fn TpAllocAlpcCompletion (AlpcReturn : * mut PTP_ALPC , AlpcPort : HANDLE , Callback : PTP_ALPC_CALLBACK , Context : PVOID , CallbackEnviron : PTP_CALLBACK_ENVIRON ,) -> NTSTATUS ; fn TpAllocAlpcCompletionEx (AlpcReturn : * mut PTP_ALPC , AlpcPort : HANDLE , Callback : PTP_ALPC_CALLBACK_EX , Context : PVOID , CallbackEnviron : PTP_CALLBACK_ENVIRON ,) -> NTSTATUS ; fn TpReleaseAlpcCompletion (Alpc : PTP_ALPC ,) ; fn TpWaitForAlpcCompletion (Alpc : PTP_ALPC ,) ; } }
};
}
