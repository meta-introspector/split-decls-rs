// Generated macro for escape_html (function)
macro_rules! Depcrateescape_html {
() => {
// Module: crate
// Provides: {"escape_html"}
// Dependencies: {}
# [doc = " Escape tags in such a way that it is suitable for inclusion in a"] # [doc = " Graphviz HTML label."] pub fn escape_html (s : & str) -> String { s . replace ('&' , "&amp;") . replace ('\"' , "&quot;") . replace ('<' , "&lt;") . replace ('>' , "&gt;") . replace ('\n' , "<br align=\"left\"/>") }
};
}
