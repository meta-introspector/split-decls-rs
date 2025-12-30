// Generated macro for macro_2365 (macro)
macro_rules! Depcrate_ntrtlmacro_2365 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2365"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlInitializeBitMap (BitMapHeader : PRTL_BITMAP , BitMapBuffer : PULONG , SizeOfBitMap : ULONG ,) ; fn RtlClearBit (BitMapHeader : PRTL_BITMAP , BitNumber : ULONG ,) ; fn RtlSetBit (BitMapHeader : PRTL_BITMAP , BitNumber : ULONG ,) ; fn RtlTestBit (BitMapHeader : PRTL_BITMAP , BitNumber : ULONG ,) -> BOOLEAN ; fn RtlClearAllBits (BitMapHeader : PRTL_BITMAP ,) ; fn RtlSetAllBits (BitMapHeader : PRTL_BITMAP ,) ; fn RtlFindClearBits (BitMapHeader : PRTL_BITMAP , NumberToFind : ULONG , HintIndex : ULONG ,) -> ULONG ; fn RtlFindSetBits (BitMapHeader : PRTL_BITMAP , NumberToFind : ULONG , HintIndex : ULONG ,) -> ULONG ; fn RtlFindClearBitsAndSet (BitMapHeader : PRTL_BITMAP , NumberToFind : ULONG , HintIndex : ULONG ,) -> ULONG ; fn RtlFindSetBitsAndClear (BitMapHeader : PRTL_BITMAP , NumberToFind : ULONG , HintIndex : ULONG ,) -> ULONG ; fn RtlClearBits (BitMapHeader : PRTL_BITMAP , StartingIndex : ULONG , NumberToClear : ULONG ,) ; fn RtlSetBits (BitMapHeader : PRTL_BITMAP , StartingIndex : ULONG , NumberToSet : ULONG ,) ; fn RtlFindMostSignificantBit (Set : ULONGLONG ,) -> CCHAR ; fn RtlFindLeastSignificantBit (Set : ULONGLONG ,) -> CCHAR ; } }
};
}
