// Generated macro for impl_10117 (impl)
macro_rules! Depcrate_swap_ptr_to_refimpl_10117 {
() => {
// Module: crate::swap_ptr_to_ref
// Provides: {"impl_10117"}
// Dependencies: {}
impl LateLintPass < '_ > for SwapPtrToRef { fn check_expr (& mut self , cx : & LateContext < '_ > , e : & Expr < '_ >) { if let ExprKind :: Call (fn_expr , [arg1 , arg2]) = e . kind && fn_expr . basic_res () . is_diag_item (cx , sym :: mem_swap) && let ctxt = e . span . ctxt () && let (from_ptr1 , arg1_span) = is_ptr_to_ref (cx , arg1 , ctxt) && let (from_ptr2 , arg2_span) = is_ptr_to_ref (cx , arg2 , ctxt) && (from_ptr1 || from_ptr2) { span_lint_and_then (cx , SWAP_PTR_TO_REF , e . span , "call to `core::mem::swap` with a parameter derived from a raw pointer" , | diag | { if ! ((from_ptr1 && arg1_span . is_none ()) || (from_ptr2 && arg2_span . is_none ())) { let mut app = Applicability :: MachineApplicable ; let snip1 = snippet_with_context (cx , arg1_span . unwrap_or (arg1 . span) , ctxt , ".." , & mut app) . 0 ; let snip2 = snippet_with_context (cx , arg2_span . unwrap_or (arg2 . span) , ctxt , ".." , & mut app) . 0 ; diag . span_suggestion (e . span , "use ptr::swap" , format ! ("core::ptr::swap({snip1}, {snip2})") , app ,) ; } } ,) ; } } }
};
}
