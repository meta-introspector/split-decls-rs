macro_rules! UnusedResult {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_result)] pub (crate) struct UnusedResult < 'a > { pub ty : Ty < 'a > , }
    };
}

UnusedResult!()