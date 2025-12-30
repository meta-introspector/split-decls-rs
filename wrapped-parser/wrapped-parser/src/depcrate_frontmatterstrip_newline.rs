// Generated macro for strip_newline (function)
macro_rules! Depcrate_frontmatterstrip_newline {
() => {
// Module: crate::frontmatter
// Provides: {"strip_newline"}
// Dependencies: {}
fn strip_newline (text : & str) -> & str { text . strip_suffix ("\r\n") . or_else (| | text . strip_suffix ('\n')) . unwrap_or (text) }
};
}
