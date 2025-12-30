// Generated macro for macro_420 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_line_breakmacro_420 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::line_break
// Provides: {"macro_420"}
// Dependencies: {}
enum_keyword ! (# [doc = " A Unicode Line Break Style Identifier defines a preferred line break style corresponding to the CSS level 3 line-break option."] # [doc = ""] # [doc = " Specifying \"lb\" in a locale identifier overrides the locale’s default style"] # [doc = " (which may correspond to \"normal\" or \"strict\")."] # [doc = ""] # [doc = " The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeLineBreakStyleIdentifier)."] LineBreakStyle { # [doc = " CSS level 3 line-break=strict, e.g. treat CJ as NS"] ("strict" => Strict) , # [doc = " CSS level 3 line-break=normal, e.g. treat CJ as ID, break before hyphens for ja,zh"] ("normal" => Normal) , # [doc = " CSS lev 3 line-break=loose"] ("loose" => Loose) , } , "lb") ;
};
}
