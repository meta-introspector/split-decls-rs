// Generated macro for Parser (struct)
macro_rules! Depcrate_transliterate_compile_parseParser {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"Parser"}
// Dependencies: {}
pub (crate) struct Parser < 'a , P : ? Sized > { iter : Peekable < CharIndices < 'a > > , source : & 'a str , variable_map : VariableMap < 'static > , dot_set : Option < UnicodeSet > , xid_start : & 'a CodePointInversionList < 'a > , xid_continue : & 'a CodePointInversionList < 'a > , pat_ws : & 'a CodePointInversionList < 'a > , property_provider : & 'a P , }
};
}
