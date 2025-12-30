// Generated macro for macro_2139 (macro)
macro_rules! Depcrate_ntrtlmacro_2139 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2139"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlInitUnicodeString (DestinationString : PUNICODE_STRING , SourceString : PCWSTR ,) ; fn RtlInitUnicodeStringEx (DestinationString : PUNICODE_STRING , SourceString : PCWSTR ,) -> NTSTATUS ; fn RtlCreateUnicodeString (DestinationString : PUNICODE_STRING , SourceString : PCWSTR ,) -> BOOLEAN ; fn RtlCreateUnicodeStringFromAsciiz (DestinationString : PUNICODE_STRING , SourceString : PSTR ,) -> BOOLEAN ; fn RtlFreeUnicodeString (UnicodeString : PUNICODE_STRING ,) ; } }
};
}
