// Generated macro for LongRunningWarn (struct)
macro_rules! Depcrate_errorsLongRunningWarn {
() => {
// Module: crate::errors
// Provides: {"LongRunningWarn"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_long_running)] pub struct LongRunningWarn { # [primary_span] # [label] pub span : Span , # [help] pub item_span : Span , pub force_duplicate : usize , }
};
}
