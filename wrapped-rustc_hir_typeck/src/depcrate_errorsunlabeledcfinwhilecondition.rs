// Generated macro for UnlabeledCfInWhileCondition (struct)
macro_rules! Depcrate_errorsUnlabeledCfInWhileCondition {
() => {
// Module: crate::errors
// Provides: {"UnlabeledCfInWhileCondition"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_unlabeled_cf_in_while_condition , code = E0590)] pub (crate) struct UnlabeledCfInWhileCondition < 'a > { # [primary_span] # [label] pub span : Span , pub cf_type : & 'a str , }
};
}
