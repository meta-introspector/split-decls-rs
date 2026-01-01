// SRC: ../rust/compiler/rustc_infer/src/errors.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14 */
use rustc_macros::Diagnostic;
use crate::rustc_complete::Span;

#[derive(Diagnostic)]
#[diag(infer_opaque_hidden_type)]
pub(crate) struct OpaqueHiddenTypeDiag {
    #[primary_span]
    #[label]
    pub span: Span,
    #[note(infer_opaque_type)]
    pub opaque_type: Span,
    #[note(infer_hidden_type)]
    pub hidden_type: Span,
}