macro_rules! deps {
    () => {
        UnknownLintSuggestion!();
    };
}

macro_rules! UnknownLint {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_unknown_lint)] pub (crate) struct UnknownLint { pub name : String , # [subdiagnostic] pub suggestion : Option < UnknownLintSuggestion > , }
    };
}

UnknownLint!()