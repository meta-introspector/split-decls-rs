// Generated macro for macro_2977 (macro)
macro_rules! Depcrate_ntxcapimacro_2977 {
() => {
// Module: crate::ntxcapi
// Provides: {"macro_2977"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlDispatchException (ExceptionRecord : PEXCEPTION_RECORD , ContextRecord : PCONTEXT ,) -> BOOLEAN ; fn RtlRaiseStatus (Status : NTSTATUS ,) ; fn RtlRaiseException (ExceptionRecord : PEXCEPTION_RECORD ,) ; fn NtContinue (ContextRecord : PCONTEXT , TestAlert : BOOLEAN ,) -> NTSTATUS ; fn NtRaiseException (ExceptionRecord : PEXCEPTION_RECORD , ContextRecord : PCONTEXT , FirstChance : BOOLEAN ,) -> NTSTATUS ; fn RtlAssert (VoidFailedAssertion : PVOID , VoidFileName : PVOID , LineNumber : ULONG , MutableMessage : PSTR ,) ; } }
};
}
