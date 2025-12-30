// Generated macro for impl_446 (impl)
macro_rules! Depcrate_interpret_eval_contextimpl_446 {
() => {
// Module: crate::interpret::eval_context
// Provides: {"impl_446"}
// Dependencies: {}
impl < 'tcx , M : Machine < 'tcx > > LayoutOfHelpers < 'tcx > for InterpCx < 'tcx , M > { type LayoutOfResult = Result < TyAndLayout < 'tcx > , InterpErrorKind < 'tcx > > ; # [inline] fn layout_tcx_at_span (& self) -> Span { self . tcx . span } # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , _ : Span , _ : Ty < 'tcx > ,) -> InterpErrorKind < 'tcx > { err_inval ! (Layout (err)) } }
};
}
