macro_rules! deps {
    () => {
        RequestedLevel!();
    };
}

macro_rules! DeprecatedLintNameFromCommandLine {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_deprecated_lint_name)] # [help] pub (crate) struct DeprecatedLintNameFromCommandLine < 'a > { pub name : String , pub replace : & 'a str , # [subdiagnostic] pub requested_level : RequestedLevel < 'a > , }
    };
}

DeprecatedLintNameFromCommandLine!()