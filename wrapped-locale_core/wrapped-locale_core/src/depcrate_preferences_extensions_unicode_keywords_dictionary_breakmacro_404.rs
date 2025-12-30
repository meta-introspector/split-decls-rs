// Generated macro for macro_404 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_dictionary_breakmacro_404 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::dictionary_break
// Provides: {"macro_404"}
// Dependencies: {}
struct_keyword ! (# [doc = " A Unicode Dictionary Break Exclusion Identifier specifies"] # [doc = " scripts to be excluded from dictionary-based text break (for words and lines)."] # [doc = ""] # [doc = " The valid values are of one or more items of type [`Script`](crate::subtags::Script)."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] DictionaryBreakScriptExclusions , "dx" , Vec < Script >, | input : Value | { input . into_iter () . map (| s | { Script :: from_str (s . as_str ()) . map_err (| _ | PreferencesParseError :: InvalidKeywordValue) }) . collect ::< Result < _ , _ >> () . map (Self) } , | input : DictionaryBreakScriptExclusions | { input . 0 . into_iter () . map (Into :: into) . collect () }) ;
};
}
