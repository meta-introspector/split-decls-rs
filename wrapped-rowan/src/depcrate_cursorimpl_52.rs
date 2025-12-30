// Generated macro for impl_52 (impl)
macro_rules! Depcrate_cursorimpl_52 {
() => {
// Module: crate::cursor
// Provides: {"impl_52"}
// Dependencies: {}
impl < F : Fn (SyntaxKind) -> bool > Iterator for SyntaxElementChildrenByKind < F > { type Item = SyntaxElement ; fn next (& mut self) -> Option < SyntaxElement > { self . next . take () . inspect (| next | { self . next = next . next_sibling_or_token_by_kind (& self . matcher) ; }) } }
};
}
