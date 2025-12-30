// Generated macro for macro_2368 (macro)
macro_rules! Depcrate_ntrtlmacro_2368 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2368"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlFindClearRuns (BitMapHeader : PRTL_BITMAP , RunArray : PRTL_BITMAP_RUN , SizeOfRunArray : ULONG , LocateLongestRuns : BOOLEAN ,) -> ULONG ; fn RtlFindLongestRunClear (BitMapHeader : PRTL_BITMAP , StartingIndex : PULONG ,) -> ULONG ; fn RtlFindFirstRunClear (BitMapHeader : PRTL_BITMAP , StartingIndex : PULONG ,) -> ULONG ; } }
};
}
