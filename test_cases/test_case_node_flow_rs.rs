// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/coverage/counters/node_flow.rs
// Error: expected square brackets
// Problematic line: line 16

pub(crate) use rustc_middle::mir::coverage::NodeFlowData;
use rustc_middle::mir::coverage::Op;

#[cfg(test)]
mod tests;

/// Creates a "merged" view of an underlying graph.
