// Generated macro for get_size_of_ty (function)
macro_rules! Depcrate_size_of_in_element_countget_size_of_ty {
() => {
// Module: crate::size_of_in_element_count
// Provides: {"get_size_of_ty"}
// Dependencies: {}
fn get_size_of_ty < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , inverted : bool) -> Option < Ty < 'tcx > > { match expr . kind { ExprKind :: Call (count_func , _) => { if ! inverted && let ExprKind :: Path (ref count_func_qpath) = count_func . kind && let Some (def_id) = cx . qpath_res (count_func_qpath , count_func . hir_id) . opt_def_id () && matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: mem_size_of | sym :: mem_size_of_val)) { cx . typeck_results () . node_args (count_func . hir_id) . types () . next () } else { None } } , ExprKind :: Binary (op , left , right) if BinOpKind :: Mul == op . node => { get_size_of_ty (cx , left , inverted) . or_else (| | get_size_of_ty (cx , right , inverted)) } , ExprKind :: Binary (op , left , right) if BinOpKind :: Div == op . node => { get_size_of_ty (cx , left , inverted) . or_else (| | get_size_of_ty (cx , right , ! inverted)) } , ExprKind :: Cast (expr , _) => get_size_of_ty (cx , expr , inverted) , _ => None , } }
};
}
