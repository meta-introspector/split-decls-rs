// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/ssa.rs
// Error: expected square brackets
// Problematic line: line 19

use rustc_middle::ty::{self, TyCtxt};
use tracing::{debug, instrument, trace};

pub(super) struct SsaLocals {
    /// Assignments to each local. This defines whether the local is SSA.
    assignments: IndexVec<Local, Set1<DefLocation>>,
    /// We visit the body in reverse postorder, to ensure each local is assigned before it is used.
