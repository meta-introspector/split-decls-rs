// Generated macro for impl_3590 (impl)
macro_rules! Depcrate_loops_infinite_loopimpl_3590 {
() => {
// Module: crate::loops::infinite_loop
// Provides: {"impl_3590"}
// Dependencies: {}
impl < 'hir > Visitor < 'hir > for LoopVisitor < 'hir , '_ > { fn visit_expr (& mut self , ex : & 'hir Expr < '_ >) { match & ex . kind { ExprKind :: Break (hir :: Destination { label , .. } , ..) => { if self . loop_depth == 0 || (label . is_some () && * label == self . label) { self . is_finite = true ; } } , ExprKind :: Continue (hir :: Destination { label , .. }) => { if label . is_some_and (| label | ! self . inner_labels . contains (& label)) { self . is_finite = true ; } } , ExprKind :: Ret (..) => self . is_finite = true , ExprKind :: Loop (_ , label , _ , _) => { if let Some (label) = label { self . inner_labels . push (* label) ; } self . loop_depth += 1 ; walk_expr (self , ex) ; self . loop_depth -= 1 ; if label . is_some () { self . inner_labels . pop () ; } } , _ => { if let Some (did) = fn_def_id (self . cx , ex) { let fn_ret_ty = self . cx . tcx . fn_sig (did) . skip_binder () . output () . skip_binder () ; if fn_ret_ty . is_never () { self . is_finite = true ; return ; } } walk_expr (self , ex) ; } , } } }
};
}
