// Generated macro for macro_2282 (macro)
macro_rules! Depcrate_ntrtlmacro_2282 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2282"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateHeap (Flags : ULONG , HeapBase : PVOID , ReserveSize : SIZE_T , CommitSize : SIZE_T , Lock : PVOID , Parameters : PRTL_HEAP_PARAMETERS ,) -> PVOID ; fn RtlDestroyHeap (HeapHandle : PVOID ,) -> PVOID ; fn RtlAllocateHeap (HeapHandle : PVOID , Flags : ULONG , Size : SIZE_T ,) -> PVOID ; fn RtlFreeHeap (HeapHandle : PVOID , Flags : ULONG , BaseAddress : PVOID ,) -> BOOLEAN ; fn RtlSizeHeap (HeapHandle : PVOID , Flags : ULONG , BaseAddress : PVOID ,) -> SIZE_T ; fn RtlZeroHeap (HeapHandle : PVOID , Flags : ULONG ,) -> NTSTATUS ; fn RtlProtectHeap (HeapHandle : PVOID , MakeReadOnly : BOOLEAN ,) ; } }
};
}
