// Generated macro for IfLetRescopeLint (struct)
macro_rules! Depcrate_if_let_rescopeIfLetRescopeLint {
() => {
// Module: crate::if_let_rescope
// Provides: {"IfLetRescopeLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_if_let_rescope)] struct IfLetRescopeLint { # [subdiagnostic] destructors : Vec < DestructorLabel > , # [label] significant_droppers : Vec < Span > , # [help] lifetime_ends : Vec < Span > , # [subdiagnostic] rewrite : Option < IfLetRescopeRewrite > , }
};
}
