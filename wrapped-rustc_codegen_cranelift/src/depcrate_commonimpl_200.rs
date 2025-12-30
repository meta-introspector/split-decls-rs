// Generated macro for impl_200 (impl)
macro_rules! Depcrate_commonimpl_200 {
() => {
// Module: crate::common
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'tcx > LayoutOfHelpers < 'tcx > for FullyMonomorphizedLayoutCx < 'tcx > { # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { if let LayoutError :: SizeOverflow (_) | LayoutError :: ReferencesError (_) = err { self . 0 . sess . dcx () . span_fatal (span , err . to_string ()) } else { self . 0 . sess . dcx () . span_fatal (span , format ! ("failed to get layout for `{}`: {}" , ty , err)) } } }
};
}
