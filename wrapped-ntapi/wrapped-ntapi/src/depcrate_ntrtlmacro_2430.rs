// Generated macro for macro_2430 (macro)
macro_rules! Depcrate_ntrtlmacro_2430 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2430"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlGetUnloadEventTrace () -> PRTL_UNLOAD_EVENT_TRACE ; fn RtlGetUnloadEventTraceEx (ElementSize : * mut PULONG , ElementCount : * mut PULONG , EventTrace : * mut PVOID ,) ; fn RtlQueryPerformanceCounter (PerformanceCounter : PLARGE_INTEGER ,) -> LOGICAL ; fn RtlQueryPerformanceFrequency (PerformanceFrequency : PLARGE_INTEGER ,) -> LOGICAL ; } }
};
}
