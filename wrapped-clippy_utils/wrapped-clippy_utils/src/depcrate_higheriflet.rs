// Generated macro for IfLet (struct)
macro_rules! Depcrate_higherIfLet {
() => {
// Module: crate::higher
// Provides: {"IfLet"}
// Dependencies: {}
# [doc = " An `if let` expression"] pub struct IfLet < 'hir > { # [doc = " `if let` pattern"] pub let_pat : & 'hir Pat < 'hir > , # [doc = " `if let` scrutinee"] pub let_expr : & 'hir Expr < 'hir > , # [doc = " `if let` then expression"] pub if_then : & 'hir Expr < 'hir > , # [doc = " `if let` else expression"] pub if_else : Option < & 'hir Expr < 'hir > > , # [doc = " `if let PAT = EXPR`"] # [doc = "     ^^^^^^^^^^^^^^"] pub let_span : Span , }
};
}
