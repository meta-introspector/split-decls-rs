// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs
// Error: expected square brackets
// Problematic line: line 25

use super::{SelectionCandidateSet, SelectionContext, TraitObligationStack};
use crate::traits::util;

impl<'cx, 'tcx> SelectionContext<'cx, 'tcx> {
    #[instrument(skip(self, stack), level = "debug")]
    pub(super) fn assemble_candidates<'o>(
        &mut self,
