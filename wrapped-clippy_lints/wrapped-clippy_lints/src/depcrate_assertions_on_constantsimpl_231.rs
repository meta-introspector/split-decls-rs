// Generated macro for impl_231 (impl)
macro_rules! Depcrate_assertions_on_constantsimpl_231 {
() => {
// Module: crate::assertions_on_constants
// Provides: {"impl_231"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for AssertionsOnConstants { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if let Some (macro_call) = root_macro_call_first_node (cx , e) && let is_debug = match cx . tcx . get_diagnostic_name (macro_call . def_id) { Some (sym :: debug_assert_macro) => true , Some (sym :: assert_macro) => false , _ => return , } && let Some ((condition , _)) = find_assert_args (cx , e , macro_call . expn) && let Some ((Constant :: Bool (assert_val) , const_src)) = ConstEvalCtxt :: new (cx) . eval_with_source (condition , macro_call . span . ctxt ()) && let in_const_context = is_inside_always_const_context (cx . tcx , e . hir_id) && (const_src . is_local () || ! in_const_context) && ! (is_debug && as_bool_lit (condition) == Some (false)) { let (msg , help) = if ! const_src . is_local () { let help = if self . msrv . meets (cx , msrvs :: CONST_BLOCKS) { "consider moving this into a const block: `const { assert!(..) }`" } else if self . msrv . meets (cx , msrvs :: CONST_PANIC) { "consider moving this to an anonymous constant: `const _: () = { assert!(..); }`" } else { return ; } ; ("this assertion has a constant value" , help) } else if assert_val { ("this assertion is always `true`" , "remove the assertion") } else { ("this assertion is always `false`" , "replace this with `panic!()` or `unreachable!()`" ,) } ; span_lint_and_help (cx , ASSERTIONS_ON_CONSTANTS , macro_call . span , msg , None , help) ; } } }
};
}
