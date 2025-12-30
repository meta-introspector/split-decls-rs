// Generated macro for impl_229 (impl)
macro_rules! Depcrate_assertions_on_constantsimpl_229 {
() => {
// Module: crate::assertions_on_constants
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for AssertionsOnConstants { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { let Some (macro_call) = root_macro_call_first_node (cx , e) else { return ; } ; let is_debug = match cx . tcx . get_diagnostic_name (macro_call . def_id) { Some (sym :: debug_assert_macro) => true , Some (sym :: assert_macro) => false , _ => return , } ; let Some ((condition , panic_expn)) = find_assert_args (cx , e , macro_call . expn) else { return ; } ; let Some (Constant :: Bool (val)) = ConstEvalCtxt :: new (cx) . eval (condition) else { return ; } ; match condition . kind { ExprKind :: Path (..) | ExprKind :: Lit (_) => { } , _ if is_inside_always_const_context (cx . tcx , e . hir_id) => return , _ => { } , } if val { span_lint_and_help (cx , ASSERTIONS_ON_CONSTANTS , macro_call . span , format ! ("`{}!(true)` will be optimized out by the compiler" , cx . tcx . item_name (macro_call . def_id)) , None , "remove it" ,) ; } else if ! is_debug { let (assert_arg , panic_arg) = match panic_expn { PanicExpn :: Empty => ("" , "") , _ => (", .." , "..") , } ; span_lint_and_help (cx , ASSERTIONS_ON_CONSTANTS , macro_call . span , format ! ("`assert!(false{assert_arg})` should probably be replaced") , None , format ! ("use `panic!({panic_arg})` or `unreachable!({panic_arg})`") ,) ; } } }
};
}
