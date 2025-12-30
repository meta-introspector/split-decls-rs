// Generated macro for reindent_multiline (function)
macro_rules! Depcrate_sourcereindent_multiline {
() => {
// Module: crate::source
// Provides: {"reindent_multiline"}
// Dependencies: {}
# [doc = " Reindent a multiline string with possibility of ignoring the first line."] pub fn reindent_multiline (s : & str , ignore_first : bool , indent : Option < usize >) -> String { let s_space = reindent_multiline_inner (s , ignore_first , indent , ' ') ; let s_tab = reindent_multiline_inner (& s_space , ignore_first , indent , '\t') ; reindent_multiline_inner (& s_tab , ignore_first , indent , ' ') }
};
}
