// Generated macro for macro_2370 (macro)
macro_rules! Depcrate_ntrtlmacro_2370 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2370"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlNumberOfClearBits (BitMapHeader : PRTL_BITMAP ,) -> ULONG ; fn RtlNumberOfSetBits (BitMapHeader : PRTL_BITMAP ,) -> ULONG ; fn RtlAreBitsClear (BitMapHeader : PRTL_BITMAP , StartingIndex : ULONG , Length : ULONG ,) -> BOOLEAN ; fn RtlAreBitsSet (BitMapHeader : PRTL_BITMAP , StartingIndex : ULONG , Length : ULONG ,) -> BOOLEAN ; fn RtlFindNextForwardRunClear (BitMapHeader : PRTL_BITMAP , FromIndex : ULONG , StartingRunIndex : PULONG ,) -> ULONG ; fn RtlFindLastBackwardRunClear (BitMapHeader : PRTL_BITMAP , FromIndex : ULONG , StartingRunIndex : PULONG ,) -> ULONG ; fn RtlNumberOfSetBitsUlongPtr (Target : ULONG_PTR ,) -> ULONG ; fn RtlInterlockedClearBitRun (BitMapHeader : PRTL_BITMAP , StartingIndex : ULONG , NumberToClear : ULONG ,) ; fn RtlInterlockedSetBitRun (BitMapHeader : PRTL_BITMAP , StartingIndex : ULONG , NumberToSet : ULONG ,) ; fn RtlCopyBitMap (Source : PRTL_BITMAP , Destination : PRTL_BITMAP , TargetBit : ULONG ,) ; fn RtlExtractBitMap (Source : PRTL_BITMAP , Destination : PRTL_BITMAP , TargetBit : ULONG , NumberOfBits : ULONG ,) ; fn RtlNumberOfClearBitsInRange (BitMapHeader : PRTL_BITMAP , StartingIndex : ULONG , Length : ULONG ,) -> ULONG ; fn RtlNumberOfSetBitsInRange (BitMapHeader : PRTL_BITMAP , StartingIndex : ULONG , Length : ULONG ,) -> ULONG ; } }
};
}
