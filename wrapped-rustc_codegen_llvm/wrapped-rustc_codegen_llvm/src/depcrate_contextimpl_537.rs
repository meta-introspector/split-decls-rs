// Generated macro for impl_537 (impl)
macro_rules! Depcrate_contextimpl_537 {
() => {
// Module: crate::context
// Provides: {"impl_537"}
// Dependencies: {}
impl < 'tcx > FnAbiOfHelpers < 'tcx > for CodegenCx < '_ , 'tcx > { # [inline] fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , span : Span , fn_abi_request : FnAbiRequest < 'tcx > ,) -> ! { match err { FnAbiError :: Layout (LayoutError :: SizeOverflow (_) | LayoutError :: Cycle (_)) => { self . tcx . dcx () . emit_fatal (Spanned { span , node : err }) ; } _ => match fn_abi_request { FnAbiRequest :: OfFnPtr { sig , extra_args } => { span_bug ! (span , "`fn_abi_of_fn_ptr({sig}, {extra_args:?})` failed: {err:?}" ,) ; } FnAbiRequest :: OfInstance { instance , extra_args } => { span_bug ! (span , "`fn_abi_of_instance({instance}, {extra_args:?})` failed: {err:?}" ,) ; } } , } } }
};
}
