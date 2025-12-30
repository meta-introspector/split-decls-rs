// Generated macro for UnstableIntrinsic (struct)
macro_rules! Depcrate_errorsUnstableIntrinsic {
() => {
// Module: crate::errors
// Provides: {"UnstableIntrinsic"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_unstable_intrinsic)] pub (crate) struct UnstableIntrinsic { # [primary_span] pub span : Span , pub name : Symbol , pub feature : Symbol , # [suggestion (const_eval_unstable_intrinsic_suggestion , code = "#![feature({feature})]\n" , applicability = "machine-applicable")] pub suggestion : Span , }
};
}
