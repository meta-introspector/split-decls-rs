// Generated macro for macro_2373 (macro)
macro_rules! Depcrate_ntrtlmacro_2373 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2373"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlInitializeBitMapEx (BitMapHeader : PRTL_BITMAP_EX , BitMapBuffer : PULONG64 , SizeOfBitMap : ULONG64 ,) ; fn RtlTestBitEx (BitMapHeader : PRTL_BITMAP_EX , BitNumber : ULONG64 ,) -> BOOLEAN ; fn RtlClearAllBitsEx (BitMapHeader : PRTL_BITMAP_EX ,) ; fn RtlClearBitEx (BitMapHeader : PRTL_BITMAP_EX , BitNumber : ULONG64 ,) ; fn RtlSetBitEx (BitMapHeader : PRTL_BITMAP_EX , BitNumber : ULONG64 ,) ; fn RtlFindSetBitsEx (BitMapHeader : PRTL_BITMAP_EX , NumberToFind : ULONG64 , HintIndex : ULONG64 ,) -> ULONG64 ; fn RtlFindSetBitsAndClearEx (BitMapHeader : PRTL_BITMAP_EX , NumberToFind : ULONG64 , HintIndex : ULONG64 ,) -> ULONG64 ; } }
};
}
