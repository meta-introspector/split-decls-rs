// Generated macro for UnallowedInlineAsm (struct)
macro_rules! Depcrate_errorsUnallowedInlineAsm {
() => {
// Module: crate::errors
// Provides: {"UnallowedInlineAsm"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_unallowed_inline_asm , code = E0015)] pub (crate) struct UnallowedInlineAsm { # [primary_span] pub span : Span , pub kind : ConstContext , }
};
}
