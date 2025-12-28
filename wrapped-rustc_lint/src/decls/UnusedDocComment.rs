macro_rules! UnusedDocComment {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_doc_comment)] # [help] pub (crate) struct UnusedDocComment { # [label] pub span : Span , }
    };
}

UnusedDocComment!();