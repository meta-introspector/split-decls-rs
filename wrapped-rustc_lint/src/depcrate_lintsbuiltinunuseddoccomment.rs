// Generated macro for BuiltinUnusedDocComment (struct)
macro_rules! Depcrate_lintsBuiltinUnusedDocComment {
() => {
// Module: crate::lints
// Provides: {"BuiltinUnusedDocComment"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_builtin_unused_doc_comment)] pub (crate) struct BuiltinUnusedDocComment < 'a > { pub kind : & 'a str , # [label] pub label : Span , # [subdiagnostic] pub sub : BuiltinUnusedDocCommentSub , }
};
}
