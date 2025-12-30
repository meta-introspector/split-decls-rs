// Generated macro for impl_4409 (impl)
macro_rules! Depcrate_manual_slice_size_calculationimpl_4409 {
() => {
// Module: crate::manual_slice_size_calculation
// Provides: {"impl_4409"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ManualSliceSizeCalculation { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: Binary (ref op , left , right) = expr . kind && BinOpKind :: Mul == op . node && ! expr . span . from_expansion () && let Some ((receiver , refs_count)) = simplify (cx , left , right) && (! is_in_const_context (cx) || self . msrv . meets (cx , msrvs :: CONST_SIZE_OF_VAL)) { let ctxt = expr . span . ctxt () ; let mut app = Applicability :: MachineApplicable ; let deref = if refs_count > 0 { "*" . repeat (refs_count - 1) } else { "&" . into () } ; let val_name = snippet_with_context (cx , receiver . span , ctxt , "slice" , & mut app) . 0 ; let Some (sugg) = std_or_core (cx) else { return } ; span_lint_and_sugg (cx , MANUAL_SLICE_SIZE_CALCULATION , expr . span , "manual slice size calculation" , "try" , format ! ("{sugg}::mem::size_of_val({deref}{val_name})") , app ,) ; } } }
};
}
