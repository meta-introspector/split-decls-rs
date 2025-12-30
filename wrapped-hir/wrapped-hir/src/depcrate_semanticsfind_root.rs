// Generated macro for find_root (function)
macro_rules! Depcrate_semanticsfind_root {
() => {
// Module: crate::semantics
// Provides: {"find_root"}
// Dependencies: {}
fn find_root (node : & SyntaxNode) -> SyntaxNode { node . ancestors () . last () . unwrap () }
};
}
