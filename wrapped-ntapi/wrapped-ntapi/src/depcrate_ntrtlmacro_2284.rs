// Generated macro for macro_2284 (macro)
macro_rules! Depcrate_ntrtlmacro_2284 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2284"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlLockHeap (HeapHandle : PVOID ,) -> BOOLEAN ; fn RtlUnlockHeap (HeapHandle : PVOID ,) -> BOOLEAN ; fn RtlReAllocateHeap (HeapHandle : PVOID , Flags : ULONG , BaseAddress : PVOID , Size : SIZE_T ,) -> PVOID ; fn RtlGetUserInfoHeap (HeapHandle : PVOID , Flags : ULONG , BaseAddress : PVOID , UserValue : * mut PVOID , UserFlags : PULONG ,) -> BOOLEAN ; fn RtlSetUserValueHeap (HeapHandle : PVOID , Flags : ULONG , BaseAddress : PVOID , UserValue : PVOID ,) -> BOOLEAN ; fn RtlSetUserFlagsHeap (HeapHandle : PVOID , Flags : ULONG , BaseAddress : PVOID , UserFlagsReset : ULONG , UserFlagsSet : ULONG ,) -> BOOLEAN ; } }
};
}
