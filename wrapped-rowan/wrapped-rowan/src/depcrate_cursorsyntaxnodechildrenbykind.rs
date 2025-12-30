// Generated macro for SyntaxNodeChildrenByKind (struct)
macro_rules! Depcrate_cursorSyntaxNodeChildrenByKind {
() => {
// Module: crate::cursor
// Provides: {"SyntaxNodeChildrenByKind"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct SyntaxNodeChildrenByKind < F : Fn (SyntaxKind) -> bool > { next : Option < SyntaxNode > , matcher : F , }
};
}
