// Generated macro for check_expr_inner (function)
macro_rules! Depcrate_suspicious_trait_implcheck_expr_inner {
() => {
// Module: crate::suspicious_trait_impl
// Provides: {"check_expr_inner"}
// Dependencies: {}
fn check_expr_inner < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , binop : hir :: BinOpKind , span : Span) { if let Some ((binop_trait_lang , op_assign_trait_lang)) = binop_traits (binop) && let Some (binop_trait_id) = cx . tcx . lang_items () . get (binop_trait_lang) && let Some (op_assign_trait_id) = cx . tcx . lang_items () . get (op_assign_trait_lang) && let parent_fn = cx . tcx . hir_get_parent_item (expr . hir_id) . def_id && let hir :: Node :: ImplItem (impl_item) = cx . tcx . hir_node_by_def_id (parent_fn) && let hir :: ImplItemKind :: Fn (_ , body_id) = impl_item . kind && let body = cx . tcx . hir_body (body_id) && let parent_fn = cx . tcx . hir_get_parent_item (expr . hir_id) && let Some (trait_ref) = trait_ref_of_method (cx , parent_fn) && let trait_id = trait_ref . path . res . def_id () && ! [binop_trait_id , op_assign_trait_id] . contains (& trait_id) && let Some (& (_ , lint)) = [(& BINOP_TRAITS , SUSPICIOUS_ARITHMETIC_IMPL) , (& OP_ASSIGN_TRAITS , SUSPICIOUS_OP_ASSIGN_IMPL) ,] . iter () . find (| & (ts , _) | ts . iter () . any (| & t | Some (trait_id) == cx . tcx . lang_items () . get (t))) && count_binops (body . value) == 1 { span_lint (cx , lint , span , format ! ("suspicious use of `{}` in `{}` impl" , binop . as_str () , cx . tcx . item_name (trait_id)) ,) ; } }
};
}
