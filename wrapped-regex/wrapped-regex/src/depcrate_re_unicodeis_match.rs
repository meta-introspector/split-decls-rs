// Generated macro for is_match (function)
macro_rules! Depcrate_re_unicodeis_match {
() => {
// Module: crate::re_unicode
// Provides: {"is_match"}
// Dependencies: {}
# [doc = " Tests if the given regular expression matches somewhere in the text given."] # [doc = ""] # [doc = " If there was a problem compiling the regular expression, an error is"] # [doc = " returned."] # [doc = ""] # [doc = " To find submatches, split or replace text, you'll need to compile an"] # [doc = " expression first."] pub fn is_match (regex : & str , text : & str) -> Result < bool , Error > { Regex :: new (regex) . map (| r | r . is_match (text)) }
};
}
