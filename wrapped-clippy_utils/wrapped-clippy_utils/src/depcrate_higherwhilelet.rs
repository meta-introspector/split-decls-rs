// Generated macro for WhileLet (struct)
macro_rules! Depcrate_higherWhileLet {
() => {
// Module: crate::higher
// Provides: {"WhileLet"}
// Dependencies: {}
# [doc = " A desugared `while let` loop"] pub struct WhileLet < 'hir > { # [doc = " `while let` loop item pattern"] pub let_pat : & 'hir Pat < 'hir > , # [doc = " `while let` loop scrutinee"] pub let_expr : & 'hir Expr < 'hir > , # [doc = " `while let` loop body"] pub if_then : & 'hir Expr < 'hir > , pub label : Option < ast :: Label > , # [doc = " `while let PAT = EXPR`"] # [doc = "        ^^^^^^^^^^^^^^"] pub let_span : Span , }
};
}
