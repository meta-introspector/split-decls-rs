// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ty_utils/src/sig_types.rs
// Error: expected square brackets
// Problematic line: line 11

use rustc_span::Span;
use tracing::{instrument, trace};

pub trait SpannedTypeVisitor<'tcx> {
    type Result: VisitorResult = ();
    fn visit(&mut self, span: Span, value: impl TypeVisitable<TyCtxt<'tcx>>) -> Self::Result;
}
