// Generated macro for macro_1113 (macro)
macro_rules! Depcrate_ntldrmacro_1113 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1113"}
// Dependencies: {}
STRUCT ! { struct LDRP_LOAD_CONTEXT { BaseDllName : UNICODE_STRING , somestruct : PVOID , Flags : ULONG , pstatus : * mut NTSTATUS , ParentEntry : * mut LDR_DATA_TABLE_ENTRY , Entry : * mut LDR_DATA_TABLE_ENTRY , WorkQueueListEntry : LIST_ENTRY , ReplacedEntry : * mut LDR_DATA_TABLE_ENTRY , pvImports : * mut * mut LDR_DATA_TABLE_ENTRY , ImportDllCount : ULONG , TaskCount : LONG , pvIAT : PVOID , SizeOfIAT : ULONG , CurrentDll : ULONG , piid : PIMAGE_IMPORT_DESCRIPTOR , OriginalIATProtect : ULONG , GuardCFCheckFunctionPointer : PVOID , pGuardCFCheckFunctionPointer : * mut PVOID , } }
};
}
