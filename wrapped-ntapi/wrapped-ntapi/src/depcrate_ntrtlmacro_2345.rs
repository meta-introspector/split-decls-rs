// Generated macro for macro_2345 (macro)
macro_rules! Depcrate_ntrtlmacro_2345 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2345"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlQueryProcessDebugInformation (UniqueProcessId : HANDLE , Flags : ULONG , Buffer : PRTL_DEBUG_INFORMATION ,) -> NTSTATUS ; fn RtlFindMessage (DllHandle : PVOID , MessageTableId : ULONG , MessageLanguageId : ULONG , MessageId : ULONG , MessageEntry : * mut PMESSAGE_RESOURCE_ENTRY ,) -> NTSTATUS ; fn RtlFormatMessage (MessageFormat : PWSTR , MaximumWidth : ULONG , IgnoreInserts : BOOLEAN , ArgumentsAreAnsi : BOOLEAN , ArgumentsAreAnArray : BOOLEAN , Arguments : * mut va_list , Buffer : PWSTR , Length : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; } }
};
}
