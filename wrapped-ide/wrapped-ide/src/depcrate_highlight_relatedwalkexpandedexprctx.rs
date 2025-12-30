// Generated macro for WalkExpandedExprCtx (struct)
macro_rules! Depcrate_highlight_relatedWalkExpandedExprCtx {
() => {
// Module: crate::highlight_related
// Provides: {"WalkExpandedExprCtx"}
// Dependencies: {}
# [doc = " Preorder walk all the expression's child expressions."] # [doc = " For macro calls, the callback will be called on the expanded expressions after"] # [doc = " visiting the macro call itself."] struct WalkExpandedExprCtx < 'a > { sema : & 'a Semantics < 'a , RootDatabase > , depth : usize , check_ctx : & 'static dyn Fn (& ast :: Expr) -> bool , }
};
}
