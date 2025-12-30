// Generated macro for macro_2352 (macro)
macro_rules! Depcrate_ntrtlmacro_2352 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2352"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlFormatMessageEx (MessageFormat : PWSTR , MaximumWidth : ULONG , IgnoreInserts : BOOLEAN , ArgumentsAreAnsi : BOOLEAN , ArgumentsAreAnArray : BOOLEAN , Arguments : * mut va_list , Buffer : PWSTR , Length : ULONG , ReturnLength : PULONG , ParseContext : PPARSE_MESSAGE_CONTEXT ,) -> NTSTATUS ; fn RtlNtStatusToDosError (Status : NTSTATUS ,) -> ULONG ; fn RtlNtStatusToDosErrorNoTeb (Status : NTSTATUS ,) -> ULONG ; fn RtlGetLastNtStatus () -> NTSTATUS ; fn RtlGetLastWin32Error () -> LONG ; fn RtlSetLastWin32ErrorAndNtStatusFromNtStatus (Status : NTSTATUS ,) ; fn RtlSetLastWin32Error (Win32Error : LONG ,) ; fn RtlRestoreLastWin32Error (Win32Error : LONG ,) ; } }
};
}
