// Generated macro for UnicodeSetBuilder (struct)
macro_rules! Depcrate_unicodeset_parse_parseUnicodeSetBuilder {
() => {
// Module: crate::unicodeset_parse::parse
// Provides: {"UnicodeSetBuilder"}
// Dependencies: {}
struct UnicodeSetBuilder < 'a , 'b , P : ? Sized > { single_set : CodePointInversionListBuilder , string_set : BTreeSet < String > , iter : & 'a mut Peekable < CharIndices < 'b > > , source : & 'b str , inverted : bool , variable_map : & 'a VariableMap < 'a > , xid_start : & 'a CodePointInversionList < 'a > , xid_continue : & 'a CodePointInversionList < 'a > , pat_ws : & 'a CodePointInversionList < 'a > , property_provider : & 'a P , }
};
}
