macro_rules! deps {
    () => {
        DestructorLabel!();
        IfLetRescopeRewrite!();
    };
}

macro_rules! IfLetRescopeLint {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_if_let_rescope)] struct IfLetRescopeLint { # [subdiagnostic] destructors : Vec < DestructorLabel > , # [label] significant_droppers : Vec < Span > , # [help] lifetime_ends : Vec < Span > , # [subdiagnostic] rewrite : Option < IfLetRescopeRewrite > , }
    };
}

IfLetRescopeLint!()