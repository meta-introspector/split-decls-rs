// Generated macro for Expr (struct)
macro_rules! Depcrate_hirExpr {
() => {
// Module: crate::hir
// Provides: {"Expr"}
// Dependencies: {}
# [doc = " An expression."] # [doc = ""] # [doc = " For more details, see the [rust lang reference]."] # [doc = " Note that the reference does not document nightly-only features."] # [doc = " There may be also slight differences in the names and representation of AST nodes between"] # [doc = " the compiler and the reference."] # [doc = ""] # [doc = " [rust lang reference]: https://doc.rust-lang.org/reference/expressions.html"] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Expr < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub kind : ExprKind < 'hir > , pub span : Span , }
};
}
