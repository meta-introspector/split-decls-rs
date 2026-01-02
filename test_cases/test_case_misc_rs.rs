// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/misc.rs
// Error: expected square brackets
// Problematic line: line 12


use crate::builder::Builder;

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// Adds a new temporary value of type `ty` storing the result of
    /// evaluating `expr`.
    ///
