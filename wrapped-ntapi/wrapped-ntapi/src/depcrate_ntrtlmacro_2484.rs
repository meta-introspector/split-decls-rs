// Generated macro for macro_2484 (macro)
macro_rules! Depcrate_ntrtlmacro_2484 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2484"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlRegisterSecureMemoryCacheCallback (Callback : PRTL_SECURE_MEMORY_CACHE_CALLBACK ,) -> NTSTATUS ; fn RtlDeregisterSecureMemoryCacheCallback (Callback : PRTL_SECURE_MEMORY_CACHE_CALLBACK ,) -> NTSTATUS ; fn RtlFlushSecureMemoryCache (MemoryCache : PVOID , MemoryLength : SIZE_T ,) -> BOOLEAN ; } }
};
}
