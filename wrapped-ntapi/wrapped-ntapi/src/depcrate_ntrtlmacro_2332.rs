// Generated macro for macro_2332 (macro)
macro_rules! Depcrate_ntrtlmacro_2332 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2332"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateQueryDebugBuffer (MaximumCommit : ULONG , UseEventPair : BOOLEAN ,) -> PRTL_DEBUG_INFORMATION ; fn RtlDestroyQueryDebugBuffer (Buffer : PRTL_DEBUG_INFORMATION ,) -> NTSTATUS ; fn RtlCommitDebugInfo (Buffer : PRTL_DEBUG_INFORMATION , Size : SIZE_T ,) -> PVOID ; fn RtlDeCommitDebugInfo (Buffer : PRTL_DEBUG_INFORMATION , p : PVOID , Size : SIZE_T ,) ; } }
};
}
