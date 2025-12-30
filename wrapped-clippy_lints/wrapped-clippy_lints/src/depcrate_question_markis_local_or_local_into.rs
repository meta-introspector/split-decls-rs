// Generated macro for is_local_or_local_into (function)
macro_rules! Depcrate_question_markis_local_or_local_into {
() => {
// Module: crate::question_mark
// Provides: {"is_local_or_local_into"}
// Dependencies: {}
# [doc = " Check if `expr` is `val` or `val.into()`"] fn is_local_or_local_into (cx : & LateContext < '_ > , expr : & Expr < '_ > , val : HirId) -> bool { let is_into_call = fn_def_id_with_node_args (cx , expr) . and_then (| (fn_def_id , _) | cx . tcx . trait_of_assoc (fn_def_id)) . is_some_and (| trait_def_id | cx . tcx . is_diagnostic_item (sym :: Into , trait_def_id)) ; match expr . kind { ExprKind :: MethodCall (_ , recv , [] , _) | ExprKind :: Call (_ , [recv]) => { is_into_call && recv . res_local_id () == Some (val) } , _ => expr . res_local_id () == Some (val) , } }
};
}
