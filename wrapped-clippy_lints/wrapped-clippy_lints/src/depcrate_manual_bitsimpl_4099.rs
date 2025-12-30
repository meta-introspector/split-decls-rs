// Generated macro for impl_4099 (impl)
macro_rules! Depcrate_manual_bitsimpl_4099 {
() => {
// Module: crate::manual_bits
// Provides: {"impl_4099"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ManualBits { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Binary (bin_op , left_expr , right_expr) = expr . kind && let BinOpKind :: Mul = & bin_op . node && ! expr . span . from_expansion () && let ctxt = expr . span . ctxt () && left_expr . span . ctxt () == ctxt && right_expr . span . ctxt () == ctxt && let Some ((real_ty_span , resolved_ty , other_expr)) = get_one_size_of_ty (cx , left_expr , right_expr) && matches ! (resolved_ty . kind () , ty :: Int (_) | ty :: Uint (_)) && let ExprKind :: Lit (lit) = & other_expr . kind && let LitKind :: Int (Pu128 (8) , _) = lit . node && self . msrv . meets (cx , msrvs :: INTEGER_BITS) { let mut app = Applicability :: MachineApplicable ; let ty_snip = snippet_with_context (cx , real_ty_span , ctxt , ".." , & mut app) . 0 ; let sugg = create_sugg (cx , expr , format ! ("{ty_snip}::BITS")) ; span_lint_and_sugg (cx , MANUAL_BITS , expr . span , "usage of `size_of::<T>()` to obtain the size of `T` in bits" , "consider using" , sugg , app ,) ; } } }
};
}
