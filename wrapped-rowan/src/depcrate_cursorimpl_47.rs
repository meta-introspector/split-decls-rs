// Generated macro for impl_47 (impl)
macro_rules! Depcrate_cursorimpl_47 {
() => {
// Module: crate::cursor
// Provides: {"impl_47"}
// Dependencies: {}
impl < F : Fn (SyntaxKind) -> bool > Iterator for SyntaxNodeChildrenByKind < F > { type Item = SyntaxNode ; fn next (& mut self) -> Option < SyntaxNode > { self . next . take () . inspect (| next | { self . next = next . next_sibling_by_kind (& self . matcher) ; }) } }
};
}
