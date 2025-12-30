// Generated macro for If (struct)
macro_rules! Depcrate_higherIf {
() => {
// Module: crate::higher
// Provides: {"If"}
// Dependencies: {}
# [doc = " An `if` expression without `let`"] pub struct If < 'hir > { # [doc = " `if` condition"] pub cond : & 'hir Expr < 'hir > , # [doc = " `if` then expression"] pub then : & 'hir Expr < 'hir > , # [doc = " `else` expression"] pub r#else : Option < & 'hir Expr < 'hir > > , }
};
}
