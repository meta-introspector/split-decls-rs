// Generated macro for is_mutable (function)
macro_rules! Depcrateis_mutable {
() => {
// Module: crate
// Provides: {"is_mutable"}
// Dependencies: {}
# [doc = " Returns `true` if `expr` designates a mutable static, a mutable local binding, or an expression"] # [doc = " that can be owned."] pub fn is_mutable (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Some (hir_id) = expr . res_local_id () && let Node :: Pat (pat) = cx . tcx . hir_node (hir_id) { matches ! (pat . kind , PatKind :: Binding (BindingMode :: MUT , ..)) } else if let ExprKind :: Path (p) = & expr . kind && let Some (mutability) = cx . qpath_res (p , expr . hir_id) . opt_def_id () . and_then (| id | cx . tcx . static_mutability (id)) { mutability == Mutability :: Mut } else if let ExprKind :: Field (parent , _) = expr . kind { is_mutable (cx , parent) } else { true } }
};
}
