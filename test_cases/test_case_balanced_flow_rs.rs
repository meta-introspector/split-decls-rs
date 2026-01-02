// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/coverage/counters/balanced_flow.rs
// Error: expected square brackets
// Problematic line: line 23

use rustc_index::Idx;
use rustc_index::bit_set::DenseBitSet;

/// A view of an underlying graph that has been augmented to have “balanced flow”.
/// This means that the flow (execution count) of each node is equal to the
/// sum of its in-edge flows, and also equal to the sum of its out-edge flows.
///
