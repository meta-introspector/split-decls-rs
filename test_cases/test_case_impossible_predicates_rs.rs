// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/impossible_predicates.rs
// Error: expected square brackets
// Problematic line: line 38


pub(crate) struct ImpossiblePredicates;

impl<'tcx> MirPass<'tcx> for ImpossiblePredicates {
    #[tracing::instrument(level = "trace", skip(self, tcx, body))]
    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        tracing::trace!(def_id = ?body.source.def_id());
