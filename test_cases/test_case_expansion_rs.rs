// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/coverage/expansion.rs
// Error: unexpected token, expected `;`
// Problematic line: line 5

use rustc_middle::mir::coverage::BasicCoverageBlock;
use rustc_span::{ExpnId, ExpnKind, Span};

#[derive(Clone, Copy, Debug)]
pub(crate) struct SpanWithBcb {
    pub(crate) span: Span,
    pub(crate) bcb: BasicCoverageBlock,
