// Generated macro for macro_2146 (macro)
macro_rules! Depcrate_ntrtlmacro_2146 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2146"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlHashUnicodeString (String : PCUNICODE_STRING , CaseInSensitive : BOOLEAN , HashAlgorithm : ULONG , HashValue : PULONG ,) -> NTSTATUS ; fn RtlValidateUnicodeString (Flags : ULONG , String : PCUNICODE_STRING ,) -> NTSTATUS ; fn RtlPrefixUnicodeString (String1 : PCUNICODE_STRING , String2 : PCUNICODE_STRING , CaseInSensitive : BOOLEAN ,) -> BOOLEAN ; fn RtlSuffixUnicodeString (String1 : PUNICODE_STRING , String2 : PUNICODE_STRING , CaseInSensitive : BOOLEAN ,) -> BOOLEAN ; fn RtlFindUnicodeSubstring (FullString : PUNICODE_STRING , SearchString : PUNICODE_STRING , CaseInSensitive : BOOLEAN ,) -> PWCHAR ; } }
};
}
