// Generated macro for quote (function)
macro_rules! Depcratequote {
() => {
// Module: crate
// Provides: {"quote"}
// Dependencies: {}
# [doc = " Escapes all regular expression meta characters in `text`."] # [doc = ""] # [doc = " The string returned may be safely used as a literal in a regular"] # [doc = " expression."] pub fn quote (text : & str) -> String { let mut quoted = String :: with_capacity (text . len ()) ; for c in text . chars () { if parser :: is_punct (c) { quoted . push ('\\') ; } quoted . push (c) ; } quoted }
};
}
