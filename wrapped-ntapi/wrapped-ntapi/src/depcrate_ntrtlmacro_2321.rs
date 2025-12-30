// Generated macro for macro_2321 (macro)
macro_rules! Depcrate_ntrtlmacro_2321 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2321"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateMemoryZone (MemoryZone : * mut PVOID , InitialSize : SIZE_T , Flags : ULONG ,) -> NTSTATUS ; fn RtlDestroyMemoryZone (MemoryZone : PVOID ,) -> NTSTATUS ; fn RtlAllocateMemoryZone (MemoryZone : PVOID , BlockSize : SIZE_T , Block : * mut PVOID ,) -> NTSTATUS ; fn RtlResetMemoryZone (MemoryZone : PVOID ,) -> NTSTATUS ; fn RtlLockMemoryZone (MemoryZone : PVOID ,) -> NTSTATUS ; fn RtlUnlockMemoryZone (MemoryZone : PVOID ,) -> NTSTATUS ; fn RtlCreateMemoryBlockLookaside (MemoryBlockLookaside : * mut PVOID , Flags : ULONG , InitialSize : ULONG , MinimumBlockSize : ULONG , MaximumBlockSize : ULONG ,) -> NTSTATUS ; fn RtlDestroyMemoryBlockLookaside (MemoryBlockLookaside : PVOID ,) -> NTSTATUS ; fn RtlAllocateMemoryBlockLookaside (MemoryBlockLookaside : PVOID , BlockSize : ULONG , Block : * mut PVOID ,) -> NTSTATUS ; fn RtlFreeMemoryBlockLookaside (MemoryBlockLookaside : PVOID , Block : PVOID ,) -> NTSTATUS ; fn RtlExtendMemoryBlockLookaside (MemoryBlockLookaside : PVOID , Increment : ULONG ,) -> NTSTATUS ; fn RtlResetMemoryBlockLookaside (MemoryBlockLookaside : PVOID ,) -> NTSTATUS ; fn RtlLockMemoryBlockLookaside (MemoryBlockLookaside : PVOID ,) -> NTSTATUS ; fn RtlUnlockMemoryBlockLookaside (MemoryBlockLookaside : PVOID ,) -> NTSTATUS ; fn RtlGetCurrentTransaction () -> HANDLE ; fn RtlSetCurrentTransaction (TransactionHandle : HANDLE ,) -> LOGICAL ; } }
};
}
