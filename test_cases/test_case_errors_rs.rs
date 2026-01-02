// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ty_utils/src/errors.rs
// Error: expected square brackets
// Problematic line: line 8

use rustc_middle::ty::{GenericArg, Ty};
use rustc_span::Span;

#[derive(Diagnostic)]
#[diag(ty_utils_needs_drop_overflow)]
pub(crate) struct NeedsDropOverflow<'tcx> {
    pub query_ty: Ty<'tcx>,
