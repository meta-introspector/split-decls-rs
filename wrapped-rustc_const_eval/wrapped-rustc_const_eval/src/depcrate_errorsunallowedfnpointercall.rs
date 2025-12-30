// Generated macro for UnallowedFnPointerCall (struct)
macro_rules! Depcrate_errorsUnallowedFnPointerCall {
() => {
// Module: crate::errors
// Provides: {"UnallowedFnPointerCall"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_unallowed_fn_pointer_call)] pub (crate) struct UnallowedFnPointerCall { # [primary_span] pub span : Span , pub kind : ConstContext , }
};
}
