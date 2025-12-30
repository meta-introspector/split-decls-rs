// Generated macro for declare_early_lint_pass (macro)
macro_rules! Depcrate_passesdeclare_early_lint_pass {
() => {
// Module: crate::passes
// Provides: {"declare_early_lint_pass"}
// Dependencies: {}
macro_rules ! declare_early_lint_pass { ([] , [$ ($ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : ident : $ arg : ty) ,*) ;) *]) => (pub trait EarlyLintPass : LintPass { $ (# [inline (always)] fn $ name (& mut self , _ : & EarlyContext <'_ >, $ (_ : $ arg) ,*) { }) * }) }
};
}
