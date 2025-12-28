macro_rules! OutOfScopeMacroCalls {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_out_of_scope_macro_calls)] # [help] pub (crate) struct OutOfScopeMacroCalls { # [label] pub span : Span , pub path : String , pub location : String , }
    };
}

OutOfScopeMacroCalls!();