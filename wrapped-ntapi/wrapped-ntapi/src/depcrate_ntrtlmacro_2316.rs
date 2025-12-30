// Generated macro for macro_2316 (macro)
macro_rules! Depcrate_ntrtlmacro_2316 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2316"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlQueryHeapInformation (HeapHandle : PVOID , HeapInformationClass : HEAP_INFORMATION_CLASS , HeapInformation : PVOID , HeapInformationLength : SIZE_T , ReturnLength : PSIZE_T ,) -> NTSTATUS ; fn RtlSetHeapInformation (HeapHandle : PVOID , HeapInformationClass : HEAP_INFORMATION_CLASS , HeapInformation : PVOID , HeapInformationLength : SIZE_T ,) -> NTSTATUS ; fn RtlMultipleAllocateHeap (HeapHandle : PVOID , Flags : ULONG , Size : SIZE_T , Count : ULONG , Array : * mut PVOID ,) -> ULONG ; fn RtlMultipleFreeHeap (HeapHandle : PVOID , Flags : ULONG , Count : ULONG , Array : * mut PVOID ,) -> ULONG ; fn RtlDetectHeapLeaks () ; fn RtlFlushHeaps () ; } }
};
}
