// Generated macro for macro_2157 (macro)
macro_rules! Depcrate_ntrtlmacro_2157 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2157"}
// Dependencies: {}
EXTERN ! { extern "system" { fn PfxInitialize (PrefixTable : PPREFIX_TABLE ,) ; fn PfxInsertPrefix (PrefixTable : PPREFIX_TABLE , Prefix : PSTRING , PrefixTableEntry : PPREFIX_TABLE_ENTRY ,) -> BOOLEAN ; fn PfxRemovePrefix (PrefixTable : PPREFIX_TABLE , PrefixTableEntry : PPREFIX_TABLE_ENTRY ,) ; fn PfxFindPrefix (PrefixTable : PPREFIX_TABLE , FullName : PSTRING ,) -> PPREFIX_TABLE_ENTRY ; } }
};
}
