macro_rules! deps {
    () => {
        PatternsInFnsWithoutBodySub!();
    };
}

macro_rules! PatternsInFnsWithoutBody {
    () => {
        deps!();
        # [derive (LintDiagnostic)] pub (crate) enum PatternsInFnsWithoutBody { # [diag (lint_pattern_in_foreign)] Foreign { # [subdiagnostic] sub : PatternsInFnsWithoutBodySub , } , # [diag (lint_pattern_in_bodiless)] Bodiless { # [subdiagnostic] sub : PatternsInFnsWithoutBodySub , } , }
    };
}

PatternsInFnsWithoutBody!();