// Generated macro for While (struct)
macro_rules! Depcrate_higherWhile {
() => {
// Module: crate::higher
// Provides: {"While"}
// Dependencies: {}
# [doc = " A desugared `while` loop"] pub struct While < 'hir > { # [doc = " `while` loop condition"] pub condition : & 'hir Expr < 'hir > , # [doc = " `while` loop body"] pub body : & 'hir Expr < 'hir > , # [doc = " Span of the loop header"] pub span : Span , pub label : Option < ast :: Label > , }
};
}
