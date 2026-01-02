// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_dataflow/src/framework/results.rs
// Error: expected square brackets
// Problematic line: line 8


use super::{Analysis, ResultsCursor};

/// The results of a dataflow analysis that has converged to fixpoint. It only holds the domain
/// values at the entry of each basic block. Domain values in other parts of the block are
/// recomputed on the fly by visitors (i.e. `ResultsCursor`, or `ResultsVisitor` impls).
pub type Results<D> = IndexVec<BasicBlock, D>;
