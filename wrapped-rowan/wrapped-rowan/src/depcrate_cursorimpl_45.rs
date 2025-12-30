// Generated macro for impl_45 (impl)
macro_rules! Depcrate_cursorimpl_45 {
() => {
// Module: crate::cursor
// Provides: {"impl_45"}
// Dependencies: {}
impl Iterator for SyntaxNodeChildren { type Item = SyntaxNode ; fn next (& mut self) -> Option < SyntaxNode > { if ! self . next_initialized { self . next = self . parent . first_child () ; self . next_initialized = true ; } else { self . next = self . next . take () . and_then (| next | next . to_next_sibling ()) ; } self . next . clone () } }
};
}
