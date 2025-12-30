// Generated macro for unpack_try (function)
macro_rules! Depcrate_unused_io_amountunpack_try {
() => {
// Module: crate::unused_io_amount
// Provides: {"unpack_try"}
// Dependencies: {}
fn unpack_try < 'a > (cx : & LateContext < '_ > , mut expr : & 'a hir :: Expr < 'a >) -> & 'a hir :: Expr < 'a > { while let ExprKind :: Call (func , [arg_0]) = expr . kind && let ExprKind :: Path (qpath) = func . kind && cx . tcx . qpath_is_lang_item (qpath , hir :: LangItem :: TryTraitBranch) { expr = arg_0 ; } expr }
};
}
