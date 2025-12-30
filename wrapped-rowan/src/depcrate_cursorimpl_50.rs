// Generated macro for impl_50 (impl)
macro_rules! Depcrate_cursorimpl_50 {
() => {
// Module: crate::cursor
// Provides: {"impl_50"}
// Dependencies: {}
impl Iterator for SyntaxElementChildren { type Item = SyntaxElement ; fn next (& mut self) -> Option < SyntaxElement > { if ! self . next_initialized { self . next = self . parent . first_child_or_token () ; self . next_initialized = true ; } else { self . next = self . next . take () . and_then (| next | next . to_next_sibling_or_token ()) ; } self . next . clone () } }
};
}
