macro_rules! RemovedLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_removed_lint)] pub (crate) struct RemovedLint < 'a > { pub name : & 'a str , pub reason : & 'a str , }
    };
}

RemovedLint!()