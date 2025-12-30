// Generated macro for PatternsInFnsWithoutBody (enum)
macro_rules! Depcrate_lintsPatternsInFnsWithoutBody {
() => {
// Module: crate::lints
// Provides: {"PatternsInFnsWithoutBody"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum PatternsInFnsWithoutBody { # [diag (lint_pattern_in_foreign)] Foreign { # [subdiagnostic] sub : PatternsInFnsWithoutBodySub , } , # [diag (lint_pattern_in_bodiless)] Bodiless { # [subdiagnostic] sub : PatternsInFnsWithoutBodySub , } , }
};
}
