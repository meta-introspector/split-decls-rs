// Generated macro for impl_447 (impl)
macro_rules! Depcrate_interpret_eval_contextimpl_447 {
() => {
// Module: crate::interpret::eval_context
// Provides: {"impl_447"}
// Dependencies: {}
impl < 'tcx , M : Machine < 'tcx > > FnAbiOfHelpers < 'tcx > for InterpCx < 'tcx , M > { type FnAbiOfResult = Result < & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , InterpErrorKind < 'tcx > > ; fn handle_fn_abi_err (& self , err : FnAbiError < 'tcx > , _span : Span , _fn_abi_request : FnAbiRequest < 'tcx > ,) -> InterpErrorKind < 'tcx > { match err { FnAbiError :: Layout (err) => err_inval ! (Layout (err)) , } } }
};
}
