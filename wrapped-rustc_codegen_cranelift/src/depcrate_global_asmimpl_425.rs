// Generated macro for impl_425 (impl)
macro_rules! Depcrate_global_asmimpl_425 {
() => {
// Module: crate::global_asm
// Provides: {"impl_425"}
// Dependencies: {}
impl < 'tcx > LayoutOfHelpers < 'tcx > for GlobalAsmContext < '_ , 'tcx > { # [inline] fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { if let LayoutError :: SizeOverflow (_) | LayoutError :: ReferencesError (_) = err { self . tcx . sess . dcx () . span_fatal (span , err . to_string ()) } else { self . tcx . sess . dcx () . span_fatal (span , format ! ("failed to get layout for `{}`: {}" , ty , err)) } } }
};
}
