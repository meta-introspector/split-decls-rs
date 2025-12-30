// Generated macro for macro_2220 (macro)
macro_rules! Depcrate_ntrtlmacro_2220 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2220"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlSetUnhandledExceptionFilter (UnhandledExceptionFilter : PRTLP_UNHANDLED_EXCEPTION_FILTER ,) ; fn RtlUnhandledExceptionFilter (ExceptionPointers : PEXCEPTION_POINTERS ,) -> LONG ; fn RtlUnhandledExceptionFilter2 (ExceptionPointers : PEXCEPTION_POINTERS , Flags : ULONG ,) -> LONG ; fn RtlKnownExceptionFilter (ExceptionPointers : PEXCEPTION_POINTERS ,) -> LONG ; } }
};
}
