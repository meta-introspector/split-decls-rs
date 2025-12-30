// Generated macro for impl_315 (impl)
macro_rules! Depcrate_contextimpl_315 {
() => {
// Module: crate::context
// Provides: {"impl_315"}
// Dependencies: {}
impl < 'gcc , 'tcx > FnAbiOfHelpers < 'tcx > for CodegenCx < 'gcc , 'tcx > { # [inline] fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , span : Span , fn_abi_request : FnAbiRequest < 'tcx > ,) -> ! { if let FnAbiError :: Layout (LayoutError :: SizeOverflow (_)) = err { self . tcx . dcx () . emit_fatal (respan (span , err)) } else { match fn_abi_request { FnAbiRequest :: OfFnPtr { sig , extra_args } => { span_bug ! (span , "`fn_abi_of_fn_ptr({sig}, {extra_args:?})` failed: {err:?}") ; } FnAbiRequest :: OfInstance { instance , extra_args } => { span_bug ! (span , "`fn_abi_of_instance({instance}, {extra_args:?})` failed: {err:?}") ; } } } } }
};
}
