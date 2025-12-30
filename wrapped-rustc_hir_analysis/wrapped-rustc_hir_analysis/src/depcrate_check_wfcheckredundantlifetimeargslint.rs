// Generated macro for RedundantLifetimeArgsLint (struct)
macro_rules! Depcrate_check_wfcheckRedundantLifetimeArgsLint {
() => {
// Module: crate::check::wfcheck
// Provides: {"RedundantLifetimeArgsLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (hir_analysis_redundant_lifetime_args)] # [note] struct RedundantLifetimeArgsLint < 'tcx > { # [doc = " The lifetime we have found to be redundant."] victim : ty :: Region < 'tcx > , candidate : ty :: Region < 'tcx > , }
};
}
