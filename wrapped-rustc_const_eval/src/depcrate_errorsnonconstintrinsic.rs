// Generated macro for NonConstIntrinsic (struct)
macro_rules! Depcrate_errorsNonConstIntrinsic {
() => {
// Module: crate::errors
// Provides: {"NonConstIntrinsic"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_non_const_intrinsic)] pub (crate) struct NonConstIntrinsic { # [primary_span] pub span : Span , pub name : Symbol , pub kind : ConstContext , }
};
}
