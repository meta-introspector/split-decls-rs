// Generated macro for macro_2240 (macro)
macro_rules! Depcrate_ntrtlmacro_2240 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2240"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlDosSearchPath_Ustr (Flags : ULONG , Path : PUNICODE_STRING , FileName : PUNICODE_STRING , DefaultExtension : PUNICODE_STRING , StaticString : PUNICODE_STRING , DynamicString : PUNICODE_STRING , FullFileNameOut : * mut PCUNICODE_STRING , FilePartPrefixCch : * mut SIZE_T , BytesRequired : * mut SIZE_T ,) -> NTSTATUS ; fn RtlDoesFileExists_U (FileName : PWSTR ,) -> BOOLEAN ; fn RtlGetLengthWithoutLastFullDosOrNtPathElement (Flags : ULONG , PathString : PUNICODE_STRING , Length : PULONG ,) -> NTSTATUS ; fn RtlGetLengthWithoutTrailingPathSeperators (Flags : ULONG , PathString : PUNICODE_STRING , Length : PULONG ,) -> NTSTATUS ; } }
};
}
