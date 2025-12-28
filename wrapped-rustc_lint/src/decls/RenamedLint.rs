macro_rules! deps {
    () => {
        RenamedLintSuggestion!();
    };
}

macro_rules! RenamedLint {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_renamed_lint)] pub (crate) struct RenamedLint < 'a > { pub name : & 'a str , pub replace : & 'a str , # [subdiagnostic] pub suggestion : RenamedLintSuggestion < 'a > , }
    };
}

RenamedLint!()