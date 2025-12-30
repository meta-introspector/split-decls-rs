// Generated macro for macro_2379 (macro)
macro_rules! Depcrate_ntrtlmacro_2379 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2379"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlInitializeHandleTable (MaximumNumberOfHandles : ULONG , SizeOfHandleTableEntry : ULONG , HandleTable : PRTL_HANDLE_TABLE ,) ; fn RtlDestroyHandleTable (HandleTable : PRTL_HANDLE_TABLE ,) -> NTSTATUS ; fn RtlAllocateHandle (HandleTable : PRTL_HANDLE_TABLE , HandleIndex : PULONG ,) -> PRTL_HANDLE_TABLE_ENTRY ; fn RtlFreeHandle (HandleTable : PRTL_HANDLE_TABLE , Handle : PRTL_HANDLE_TABLE_ENTRY ,) -> BOOLEAN ; fn RtlIsValidHandle (HandleTable : PRTL_HANDLE_TABLE , Handle : PRTL_HANDLE_TABLE_ENTRY ,) -> BOOLEAN ; fn RtlIsValidIndexHandle (HandleTable : PRTL_HANDLE_TABLE , HandleIndex : ULONG , Handle : * mut PRTL_HANDLE_TABLE_ENTRY ,) -> BOOLEAN ; } }
};
}
