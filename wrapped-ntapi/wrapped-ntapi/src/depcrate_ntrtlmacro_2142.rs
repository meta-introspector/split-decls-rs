// Generated macro for macro_2142 (macro)
macro_rules! Depcrate_ntrtlmacro_2142 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2142"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlDuplicateUnicodeString (Flags : ULONG , StringIn : PCUNICODE_STRING , StringOut : PUNICODE_STRING ,) -> NTSTATUS ; fn RtlCopyUnicodeString (DestinationString : PUNICODE_STRING , SourceString : PCUNICODE_STRING ,) ; fn RtlUpcaseUnicodeChar (SourceCharacter : WCHAR ,) -> WCHAR ; fn RtlDowncaseUnicodeChar (SourceCharacter : WCHAR ,) -> WCHAR ; fn RtlCompareUnicodeString (String1 : PCUNICODE_STRING , String2 : PCUNICODE_STRING , CaseInSensitive : BOOLEAN ,) -> LONG ; fn RtlCompareUnicodeStrings (String1 : PCWCH , String1Length : SIZE_T , String2 : PCWCH , String2Length : SIZE_T , CaseInSensitive : BOOLEAN ,) -> LONG ; fn RtlEqualUnicodeString (String1 : PCUNICODE_STRING , String2 : PCUNICODE_STRING , CaseInSensitive : BOOLEAN ,) -> BOOLEAN ; } }
};
}
