// Generated macro for OverruledAttributeLint (struct)
macro_rules! Depcrate_lintsOverruledAttributeLint {
() => {
// Module: crate::lints
// Provides: {"OverruledAttributeLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_overruled_attribute)] pub (crate) struct OverruledAttributeLint < 'a > { # [label] pub overruled : Span , pub lint_level : & 'a str , pub lint_source : Symbol , # [subdiagnostic] pub sub : OverruledAttributeSub , }
};
}
