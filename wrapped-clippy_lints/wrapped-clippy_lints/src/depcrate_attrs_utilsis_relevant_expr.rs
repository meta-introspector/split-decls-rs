// Generated macro for is_relevant_expr (function)
macro_rules! Depcrate_attrs_utilsis_relevant_expr {
() => {
// Module: crate::attrs::utils
// Provides: {"is_relevant_expr"}
// Dependencies: {}
fn is_relevant_expr (cx : & LateContext < '_ > , typeck_results : & ty :: TypeckResults < '_ > , expr : & Expr < '_ >) -> bool { if macro_backtrace (expr . span) . last () . is_some_and (| macro_call | { is_panic (cx , macro_call . def_id) || cx . tcx . item_name (macro_call . def_id) == sym :: unreachable }) { return false ; } match & expr . kind { ExprKind :: Block (block , _) => is_relevant_block (cx , typeck_results , block) , ExprKind :: Ret (Some (e)) => is_relevant_expr (cx , typeck_results , e) , ExprKind :: Ret (None) | ExprKind :: Break (_ , None) => false , _ => true , } }
};
}
