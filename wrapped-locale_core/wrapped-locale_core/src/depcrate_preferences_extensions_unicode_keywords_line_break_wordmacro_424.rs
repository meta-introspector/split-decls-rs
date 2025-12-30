// Generated macro for macro_424 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_line_break_wordmacro_424 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::line_break_word
// Provides: {"macro_424"}
// Dependencies: {}
enum_keyword ! (# [doc = " A Unicode Line Break Word Identifier defines preferred line break word handling behavior corresponding to the CSS level 3 word-break option."] # [doc = ""] # [doc = " Specifying \"lw\" in a locale identifier overrides the locale’s default style (which may correspond to \"normal\" or \"keepall\")."] # [doc = ""] # [doc = " The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeLineBreakWordIdentifier)."] LineBreakWordHandling { # [doc = " CSS lev 3 word-break=normal, normal script/language behavior for midword breaks"] ("normal" => Normal) , # [doc = " CSS lev 3 word-break=break-all, allow midword breaks unless forbidden by lb setting"] ("breakall" => BreakAll) , # [doc = " CSS lev 3 word-break=keep-all, prohibit midword breaks except for dictionary breaks"] ("keepall" => KeepAll) , # [doc = " Prioritize keeping natural phrases (of multiple words) together when breaking,"] # [doc = " used in short text like title and headline"] ("phrase" => Phrase) , } , "lw") ;
};
}
