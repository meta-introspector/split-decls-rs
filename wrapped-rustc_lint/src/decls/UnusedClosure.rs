macro_rules! UnusedClosure {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_closure)] # [note] pub (crate) struct UnusedClosure < 'a > { pub count : usize , pub pre : & 'a str , pub post : & 'a str , }
    };
}

UnusedClosure!()