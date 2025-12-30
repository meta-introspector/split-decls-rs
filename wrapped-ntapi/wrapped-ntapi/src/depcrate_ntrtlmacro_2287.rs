// Generated macro for macro_2287 (macro)
macro_rules! Depcrate_ntrtlmacro_2287 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2287"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateTagHeap (HeapHandle : PVOID , Flags : ULONG , TagPrefix : PWSTR , TagNames : PWSTR ,) -> ULONG ; fn RtlQueryTagHeap (HeapHandle : PVOID , Flags : ULONG , TagIndex : USHORT , ResetCounters : BOOLEAN , TagInfo : PRTL_HEAP_TAG_INFO ,) -> PWSTR ; fn RtlExtendHeap (HeapHandle : PVOID , Flags : ULONG , Base : PVOID , Size : SIZE_T ,) -> NTSTATUS ; fn RtlCompactHeap (HeapHandle : PVOID , Flags : ULONG ,) -> SIZE_T ; fn RtlValidateHeap (HeapHandle : PVOID , Flags : ULONG , BaseAddress : PVOID ,) -> BOOLEAN ; fn RtlValidateProcessHeaps () -> BOOLEAN ; fn RtlGetProcessHeaps (NumberOfHeaps : ULONG , ProcessHeaps : * mut PVOID ,) -> ULONG ; } }
};
}
