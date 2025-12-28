macro_rules! deps {
    () => {
        UnknownLintSuggestion!();
        RequestedLevel!();
    };
}

macro_rules! UnknownLintFromCommandLine {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_unknown_lint , code = E0602)] pub (crate) struct UnknownLintFromCommandLine < 'a > { pub name : String , # [subdiagnostic] pub suggestion : Option < UnknownLintSuggestion > , # [subdiagnostic] pub requested_level : RequestedLevel < 'a > , }
    };
}

UnknownLintFromCommandLine!()