// Generated macro for escape (function)
macro_rules! Depcrate_hirescape {
() => {
// Module: crate::hir
// Provides: {"escape"}
// Dependencies: {}
# [doc = " Escapes all regular expression meta characters in `pattern`."] # [doc = ""] # [doc = " The string returned may be safely used as a literal in a regular"] # [doc = " expression."] pub fn escape (pattern : & str) -> String { let mut buf = String :: new () ; buf . reserve (pattern . len ()) ; for ch in pattern . chars () { if is_meta_character (ch) { buf . push ('\\') ; } buf . push (ch) ; } buf }
};
}
