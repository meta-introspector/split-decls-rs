macro_rules! deps {
    () => {
        ExpectationNote!();
    };
}

macro_rules! Expectation {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_expectation)] pub (crate) struct Expectation { # [subdiagnostic] pub rationale : Option < ExpectationNote > , # [note] pub note : bool , }
    };
}

Expectation!()