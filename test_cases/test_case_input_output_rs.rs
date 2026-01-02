// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/type_check/input_output.rs
// Error: expected square brackets
// Problematic line: line 24

use crate::renumber::RegionCtxt;
use crate::universal_regions::DefiningTy;

impl<'a, 'tcx> TypeChecker<'a, 'tcx> {
    /// Check explicit closure signature annotation,
    /// e.g., `|x: FxIndexMap<_, &'static u32>| ...`.
    #[instrument(skip(self), level = "debug")]
