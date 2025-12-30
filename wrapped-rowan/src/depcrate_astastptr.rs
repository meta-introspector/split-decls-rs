// Generated macro for AstPtr (struct)
macro_rules! Depcrate_astAstPtr {
() => {
// Module: crate::ast
// Provides: {"AstPtr"}
// Dependencies: {}
# [doc = " Like [`SyntaxNodePtr`], but remembers the type of node."] # [doc = ""] # [doc = " ## Note"] # [doc = " As with [`SyntaxNodePtr`], this must not be used on mutable"] # [doc = " syntax trees, since any mutation can cause the pointed node's"] # [doc = " source location to change, invalidating the pointer"] pub struct AstPtr < N : AstNode > { raw : SyntaxNodePtr < N :: Language > , }
};
}
