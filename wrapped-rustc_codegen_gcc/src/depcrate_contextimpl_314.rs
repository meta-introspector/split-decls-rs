// Generated macro for impl_314 (impl)
macro_rules! Depcrate_contextimpl_314 {
() => {
// Module: crate::context
// Provides: {"impl_314"}
// Dependencies: {}
impl < 'gcc , 'tcx > LayoutOfHelpers < 'tcx > for CodegenCx < 'gcc , 'tcx > { # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { if let LayoutError :: SizeOverflow (_) | LayoutError :: ReferencesError (_) = err { self . tcx . dcx () . emit_fatal (respan (span , err . into_diagnostic ())) } else { self . tcx . dcx () . emit_fatal (ssa_errors :: FailedToGetLayout { span , ty , err }) } } }
};
}
