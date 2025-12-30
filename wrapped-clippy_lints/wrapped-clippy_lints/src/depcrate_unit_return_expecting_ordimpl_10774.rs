// Generated macro for impl_10774 (impl)
macro_rules! Depcrate_unit_return_expecting_ordimpl_10774 {
() => {
// Module: crate::unit_return_expecting_ord
// Provides: {"impl_10774"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnitReturnExpectingOrd { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: MethodCall (_ , receiver , args , _) = expr . kind && args . iter () . any (| arg | { matches ! (arg . peel_blocks () . peel_borrows () . peel_drop_temps () . kind , ExprKind :: Path (_) | ExprKind :: Closure (_)) }) && let Some (fn_mut_trait) = cx . tcx . lang_items () . fn_mut_trait () { let ord_trait = cx . tcx . get_diagnostic_item (sym :: Ord) ; let partial_ord_trait = cx . tcx . lang_items () . partial_ord_trait () ; if (ord_trait , partial_ord_trait) == (None , None) { return ; } let args = std :: iter :: once (receiver) . chain (args . iter ()) . collect :: < Vec < _ > > () ; let arg_indices = get_args_to_check (cx , expr , args . len () , fn_mut_trait , ord_trait , partial_ord_trait) ; for (i , trait_name) in arg_indices { match check_arg (cx , args [i]) { Some ((span , None)) => { span_lint (cx , UNIT_RETURN_EXPECTING_ORD , span , format ! ("this closure returns \
                                   the unit type which also implements {trait_name}") ,) ; } , Some ((span , Some (last_semi))) => { span_lint_and_help (cx , UNIT_RETURN_EXPECTING_ORD , span , format ! ("this closure returns \
                                   the unit type which also implements {trait_name}") , Some (last_semi) , "probably caused by this trailing semicolon" ,) ; } , None => { } , } } } } }
};
}
