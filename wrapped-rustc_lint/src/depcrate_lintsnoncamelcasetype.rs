// Generated macro for NonCamelCaseType (struct)
macro_rules! Depcrate_lintsNonCamelCaseType {
() => {
// Module: crate::lints
// Provides: {"NonCamelCaseType"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_non_camel_case_type)] pub (crate) struct NonCamelCaseType < 'a > { pub sort : & 'a str , pub name : & 'a str , # [subdiagnostic] pub sub : NonCamelCaseTypeSub , }
};
}
