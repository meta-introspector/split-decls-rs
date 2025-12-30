// Generated macro for impl_7717 (impl)
macro_rules! Depcrate_mutable_debug_assertionimpl_7717 {
() => {
// Module: crate::mutable_debug_assertion
// Provides: {"impl_7717"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DebugAssertWithMutCall { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { let Some (macro_call) = root_macro_call_first_node (cx , e) else { return ; } ; if ! matches ! (cx . tcx . get_diagnostic_name (macro_call . def_id) , Some (sym :: debug_assert_macro | sym :: debug_assert_eq_macro | sym :: debug_assert_ne_macro)) { return ; } let Some ((lhs , rhs , _)) = find_assert_eq_args (cx , e , macro_call . expn) else { return ; } ; for arg in [lhs , rhs] { let mut visitor = MutArgVisitor :: new (cx) ; visitor . visit_expr (arg) ; if let Some (span) = visitor . expr_span () { span_lint (cx , DEBUG_ASSERT_WITH_MUT_CALL , span , format ! ("do not call a function with mutable arguments inside of `{}!`" , cx . tcx . item_name (macro_call . def_id)) ,) ; } } } }
};
}
