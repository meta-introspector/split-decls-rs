// Generated macro for UnallowedOpInConstContext (struct)
macro_rules! Depcrate_errorsUnallowedOpInConstContext {
() => {
// Module: crate::errors
// Provides: {"UnallowedOpInConstContext"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_unallowed_op_in_const_context)] pub (crate) struct UnallowedOpInConstContext { # [primary_span] pub span : Span , pub msg : String , }
};
}
