// Generated macro for ForLoopsOverFalliblesDiag (struct)
macro_rules! Depcrate_lintsForLoopsOverFalliblesDiag {
() => {
// Module: crate::lints
// Provides: {"ForLoopsOverFalliblesDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_for_loops_over_fallibles)] pub (crate) struct ForLoopsOverFalliblesDiag < 'a > { pub article : & 'static str , pub ref_prefix : & 'static str , pub ty : & 'static str , # [subdiagnostic] pub sub : ForLoopsOverFalliblesLoopSub < 'a > , # [subdiagnostic] pub question_mark : Option < ForLoopsOverFalliblesQuestionMark > , # [subdiagnostic] pub suggestion : ForLoopsOverFalliblesSuggestion < 'a > , }
};
}
