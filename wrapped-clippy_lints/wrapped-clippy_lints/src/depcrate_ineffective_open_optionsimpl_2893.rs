// Generated macro for impl_2893 (impl)
macro_rules! Depcrate_ineffective_open_optionsimpl_2893 {
() => {
// Module: crate::ineffective_open_options
// Provides: {"impl_2893"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for IneffectiveOpenOptions { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: MethodCall (name , recv , [_] , _) = expr . kind && name . ident . name == sym :: open && ! expr . span . from_expansion () && is_type_diagnostic_item (cx , cx . typeck_results () . expr_ty (recv) . peel_refs () , sym :: FsOpenOptions) { let mut append = false ; let mut write = None ; peel_hir_expr_while (recv , | e | { if let ExprKind :: MethodCall (name , recv , args , call_span) = e . kind && ! e . span . from_expansion () { if let [arg] = args && let ExprKind :: Lit (lit) = peel_blocks (arg) . kind && matches ! (lit . node , LitKind :: Bool (true)) && ! arg . span . from_expansion () && ! lit . span . from_expansion () { match name . ident . name { sym :: append => append = true , sym :: write if let Some (range) = call_span . map_range (cx , | _ , text , range | { if text . get (.. range . start) ? . ends_with ('.') { Some (range . start - 1 .. range . end) } else { None } }) => { write = Some (call_span . with_lo (range . start)) ; } , _ => { } , } } Some (recv) } else { None } }) ; if append && let Some (write_span) = write { span_lint_and_sugg (cx , INEFFECTIVE_OPEN_OPTIONS , write_span , "unnecessary use of `.write(true)` because there is `.append(true)`" , "remove `.write(true)`" , String :: new () , Applicability :: MachineApplicable ,) ; } } } }
};
}
