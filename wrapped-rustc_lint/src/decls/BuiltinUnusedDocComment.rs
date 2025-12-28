macro_rules! deps {
    () => {
        BuiltinUnusedDocCommentSub!();
    };
}

macro_rules! BuiltinUnusedDocComment {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_builtin_unused_doc_comment)] pub (crate) struct BuiltinUnusedDocComment < 'a > { pub kind : & 'a str , # [label] pub label : Span , # [subdiagnostic] pub sub : BuiltinUnusedDocCommentSub , }
    };
}

BuiltinUnusedDocComment!()