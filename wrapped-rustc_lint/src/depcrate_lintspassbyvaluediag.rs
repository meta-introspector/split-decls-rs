// Generated macro for PassByValueDiag (struct)
macro_rules! Depcrate_lintsPassByValueDiag {
() => {
// Module: crate::lints
// Provides: {"PassByValueDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_pass_by_value)] pub (crate) struct PassByValueDiag { pub ty : String , # [suggestion (code = "{ty}" , applicability = "maybe-incorrect")] pub suggestion : Span , }
};
}
