// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/expr/into.rs
// Error: expected square brackets
// Problematic line: line 22

use crate::builder::{BlockAnd, BlockAndExtension, BlockFrame, Builder, NeedsTemporary};
use crate::errors::{LoopMatchArmWithGuard, LoopMatchUnsupportedType};

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// Compile `expr`, storing the result into `destination`, which
    /// is assumed to be uninitialized.
    #[instrument(level = "debug", skip(self))]
