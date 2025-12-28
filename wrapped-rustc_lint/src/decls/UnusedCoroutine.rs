macro_rules! UnusedCoroutine {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_coroutine)] # [note] pub (crate) struct UnusedCoroutine < 'a > { pub count : usize , pub pre : & 'a str , pub post : & 'a str , }
    };
}

UnusedCoroutine!()