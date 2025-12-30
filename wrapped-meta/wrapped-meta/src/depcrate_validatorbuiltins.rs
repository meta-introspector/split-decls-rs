// Generated macro for BUILTINS (static)
macro_rules! Depcrate_validatorBUILTINS {
() => {
// Module: crate::validator
// Provides: {"BUILTINS"}
// Dependencies: {}
static BUILTINS : LazyLock < HashSet < & 'static str > > = LazyLock :: new (| | { ["ANY" , "DROP" , "EOI" , "PEEK" , "PEEK_ALL" , "POP" , "POP_ALL" , "SOI" , "ASCII_DIGIT" , "ASCII_NONZERO_DIGIT" , "ASCII_BIN_DIGIT" , "ASCII_OCT_DIGIT" , "ASCII_HEX_DIGIT" , "ASCII_ALPHA_LOWER" , "ASCII_ALPHA_UPPER" , "ASCII_ALPHA" , "ASCII_ALPHANUMERIC" , "ASCII" , "NEWLINE" ,] . iter () . cloned () . chain (unicode_property_names ()) . collect :: < HashSet < & str > > () }) ;
};
}
