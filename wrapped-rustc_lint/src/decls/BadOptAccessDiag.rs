macro_rules! BadOptAccessDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_bad_opt_access)] pub (crate) struct BadOptAccessDiag < 'a > { pub msg : & 'a str , }
    };
}

BadOptAccessDiag!();