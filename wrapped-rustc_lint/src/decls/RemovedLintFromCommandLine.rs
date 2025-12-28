macro_rules! deps {
    () => {
        RequestedLevel!();
    };
}

macro_rules! RemovedLintFromCommandLine {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_removed_lint)] pub (crate) struct RemovedLintFromCommandLine < 'a > { pub name : & 'a str , pub reason : & 'a str , # [subdiagnostic] pub requested_level : RequestedLevel < 'a > , }
    };
}

RemovedLintFromCommandLine!()