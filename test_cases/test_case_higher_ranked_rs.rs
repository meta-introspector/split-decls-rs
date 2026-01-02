// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/relate/higher_ranked.rs
// Error: expected square brackets
// Problematic line: line 11

use crate::infer::InferCtxt;
use crate::infer::snapshot::CombinedSnapshot;

impl<'tcx> InferCtxt<'tcx> {
    /// Replaces all bound variables (lifetimes, types, and constants) bound by
    /// `binder` with placeholder variables in a new universe. This means that the
    /// new placeholders can only be named by inference variables created after
