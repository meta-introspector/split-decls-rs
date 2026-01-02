// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/call_kind.rs
// Error: expected square brackets
// Problematic line: line 14


use crate::traits::specialization_graph;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CallDesugaringKind {
    /// for _ in x {} calls x.into_iter()
    ForLoopIntoIter,
