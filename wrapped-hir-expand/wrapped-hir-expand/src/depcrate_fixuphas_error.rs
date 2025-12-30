// Generated macro for has_error (function)
macro_rules! Depcrate_fixuphas_error {
() => {
// Module: crate::fixup
// Provides: {"has_error"}
// Dependencies: {}
fn has_error (node : & SyntaxNode) -> bool { node . children () . any (| c | c . kind () == SyntaxKind :: ERROR) }
};
}
