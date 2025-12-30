// Generated macro for NonSnakeCaseDiag (struct)
macro_rules! Depcrate_lintsNonSnakeCaseDiag {
() => {
// Module: crate::lints
// Provides: {"NonSnakeCaseDiag"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_non_snake_case)] pub (crate) struct NonSnakeCaseDiag < 'a > { pub sort : & 'a str , pub name : & 'a str , pub sc : String , # [subdiagnostic] pub sub : NonSnakeCaseDiagSub , }
};
}
