macro_rules! QueryUntracked {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_query_untracked)] # [note] pub (crate) struct QueryUntracked { pub method : Symbol , }
    };
}

QueryUntracked!()