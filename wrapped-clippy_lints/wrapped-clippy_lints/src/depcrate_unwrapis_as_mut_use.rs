// Generated macro for is_as_mut_use (function)
macro_rules! Depcrate_unwrapis_as_mut_use {
() => {
// Module: crate::unwrap
// Provides: {"is_as_mut_use"}
// Dependencies: {}
# [doc = " Checks if the parent of the expression pointed at by the given `HirId` is a call to"] # [doc = " `.as_mut()`."] # [doc = ""] # [doc = " Used by the mutation visitor to specifically allow `.as_mut()` calls."] # [doc = " In particular, the `HirId` that the visitor receives is the id of the local expression"] # [doc = " (i.e. the `x` in `x.as_mut()`), and that is the reason for why we care about its parent"] # [doc = " expression: that will be where the actual method call is."] fn is_as_mut_use (tcx : TyCtxt < '_ > , expr_id : HirId) -> bool { if let Node :: Expr (mutating_expr) = tcx . parent_hir_node (expr_id) && let ExprKind :: MethodCall (path , _ , [] , _) = mutating_expr . kind { path . ident . name == sym :: as_mut } else { false } }
};
}
