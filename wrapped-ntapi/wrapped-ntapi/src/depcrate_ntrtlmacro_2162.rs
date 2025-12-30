// Generated macro for macro_2162 (macro)
macro_rules! Depcrate_ntrtlmacro_2162 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2162"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlInitializeUnicodePrefix (PrefixTable : PUNICODE_PREFIX_TABLE ,) ; fn RtlInsertUnicodePrefix (PrefixTable : PUNICODE_PREFIX_TABLE , Prefix : PUNICODE_STRING , PrefixTableEntry : PUNICODE_PREFIX_TABLE_ENTRY ,) -> BOOLEAN ; fn RtlRemoveUnicodePrefix (PrefixTable : PUNICODE_PREFIX_TABLE , PrefixTableEntry : PUNICODE_PREFIX_TABLE_ENTRY ,) ; fn RtlFindUnicodePrefix (PrefixTable : PUNICODE_PREFIX_TABLE , FullName : PCUNICODE_STRING , CaseInsensitiveIndex : ULONG ,) -> PUNICODE_PREFIX_TABLE_ENTRY ; fn RtlNextUnicodePrefix (PrefixTable : PUNICODE_PREFIX_TABLE , Restart : BOOLEAN ,) -> PUNICODE_PREFIX_TABLE_ENTRY ; } }
};
}
