// Generated macro for macro_2106 (macro)
macro_rules! Depcrate_ntrtlmacro_2106 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2106"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlInitializeGenericTable (Table : PRTL_GENERIC_TABLE , CompareRoutine : PRTL_GENERIC_COMPARE_ROUTINE , AllocateRoutine : PRTL_GENERIC_ALLOCATE_ROUTINE , FreeRoutine : PRTL_GENERIC_FREE_ROUTINE , TableContext : PVOID ,) ; fn RtlInsertElementGenericTable (Table : PRTL_GENERIC_TABLE , Buffer : PVOID , BufferSize : CLONG , NewElement : PBOOLEAN ,) -> PVOID ; fn RtlInsertElementGenericTableFull (Table : PRTL_GENERIC_TABLE , Buffer : PVOID , BufferSize : CLONG , NewElement : PBOOLEAN , NodeOrParent : PVOID , SearchResult : TABLE_SEARCH_RESULT ,) -> PVOID ; fn RtlDeleteElementGenericTable (Table : PRTL_GENERIC_TABLE , Buffer : PVOID ,) -> BOOLEAN ; fn RtlLookupElementGenericTable (Table : PRTL_GENERIC_TABLE , Buffer : PVOID ,) -> PVOID ; fn RtlLookupElementGenericTableFull (Table : PRTL_GENERIC_TABLE , Buffer : PVOID , NodeOrParent : * mut PVOID , SearchResult : * mut TABLE_SEARCH_RESULT ,) -> PVOID ; fn RtlEnumerateGenericTable (Table : PRTL_GENERIC_TABLE , Restart : BOOLEAN ,) -> PVOID ; fn RtlEnumerateGenericTableWithoutSplaying (Table : PRTL_GENERIC_TABLE , RestartKey : * mut PVOID ,) -> PVOID ; fn RtlGetElementGenericTable (Table : PRTL_GENERIC_TABLE , I : ULONG ,) -> PVOID ; fn RtlNumberGenericTableElements (Table : PRTL_GENERIC_TABLE ,) -> ULONG ; fn RtlIsGenericTableEmpty (Table : PRTL_GENERIC_TABLE ,) -> BOOLEAN ; } }
};
}
