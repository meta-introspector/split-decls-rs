macro_rules! QueryInstability {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_query_instability)] # [note] pub (crate) struct QueryInstability { pub query : Symbol , }
    };
}

QueryInstability!()