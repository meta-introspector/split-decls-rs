macro_rules! deps {
    () => {
        RequestedLevel!();
        RenamedLintSuggestion!();
    };
}

macro_rules! RenamedLintFromCommandLine {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_renamed_lint)] pub (crate) struct RenamedLintFromCommandLine < 'a > { pub name : & 'a str , pub replace : & 'a str , # [subdiagnostic] pub suggestion : RenamedLintSuggestion < 'a > , # [subdiagnostic] pub requested_level : RequestedLevel < 'a > , }
    };
}

RenamedLintFromCommandLine!();