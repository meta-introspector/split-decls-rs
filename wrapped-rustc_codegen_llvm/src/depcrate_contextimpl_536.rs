// Generated macro for impl_536 (impl)
macro_rules! Depcrate_contextimpl_536 {
() => {
// Module: crate::context
// Provides: {"impl_536"}
// Dependencies: {}
impl < 'tcx > LayoutOfHelpers < 'tcx > for CodegenCx < '_ , 'tcx > { # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { if let LayoutError :: SizeOverflow (_) | LayoutError :: ReferencesError (_) = err { self . tcx . dcx () . emit_fatal (Spanned { span , node : err . into_diagnostic () }) } else { self . tcx . dcx () . emit_fatal (ssa_errors :: FailedToGetLayout { span , ty , err }) } } }
};
}
