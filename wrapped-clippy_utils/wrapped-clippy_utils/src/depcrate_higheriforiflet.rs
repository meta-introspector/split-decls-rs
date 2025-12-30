// Generated macro for IfOrIfLet (struct)
macro_rules! Depcrate_higherIfOrIfLet {
() => {
// Module: crate::higher
// Provides: {"IfOrIfLet"}
// Dependencies: {}
# [doc = " An `if` or `if let` expression"] pub struct IfOrIfLet < 'hir > { # [doc = " `if` condition that is maybe a `let` expression"] pub cond : & 'hir Expr < 'hir > , # [doc = " `if` then expression"] pub then : & 'hir Expr < 'hir > , # [doc = " `else` expression"] pub r#else : Option < & 'hir Expr < 'hir > > , }
};
}
