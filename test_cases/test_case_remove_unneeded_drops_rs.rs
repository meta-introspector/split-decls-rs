// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/remove_unneeded_drops.rs
// Error: expected square brackets
// Problematic line: line 22


pub(super) struct RemoveUnneededDrops;

impl<'tcx> crate::MirPass<'tcx> for RemoveUnneededDrops {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        trace!("Running RemoveUnneededDrops on {:?}", body.source);

