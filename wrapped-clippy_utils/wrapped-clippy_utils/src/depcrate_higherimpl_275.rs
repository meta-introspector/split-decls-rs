// Generated macro for impl_275 (impl)
macro_rules! Depcrate_higherimpl_275 {
() => {
// Module: crate::higher
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'a > VecArgs < 'a > { # [doc = " Returns the arguments of the `vec!` macro if this expression was expanded"] # [doc = " from `vec!`."] pub fn hir (cx : & LateContext < '_ > , expr : & 'a Expr < '_ >) -> Option < VecArgs < 'a > > { if let ExprKind :: Call (fun , args) = expr . kind && let ExprKind :: Path (ref qpath) = fun . kind && is_expn_of (fun . span , sym :: vec) . is_some () && let Some (fun_def_id) = cx . qpath_res (qpath , fun . hir_id) . opt_def_id () && let Some (name) = cx . tcx . get_diagnostic_name (fun_def_id) { return match (name , args) { (sym :: vec_from_elem , [elem , size]) => { Some (VecArgs :: Repeat (elem , size)) } , (sym :: slice_into_vec , [slice]) if let ExprKind :: Call (_ , [arg]) = slice . kind && let ExprKind :: Array (args) = arg . kind => { Some (VecArgs :: Vec (args)) } , (sym :: vec_new , []) => Some (VecArgs :: Vec (& [])) , _ => None , } ; } None } }
};
}
