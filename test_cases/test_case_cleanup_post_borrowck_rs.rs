// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/cleanup_post_borrowck.rs
// Error: expected square brackets
// Problematic line: line 26


pub(super) struct CleanupPostBorrowck;

impl<'tcx> crate::MirPass<'tcx> for CleanupPostBorrowck {
    fn run_pass(&self, _tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        for basic_block in body.basic_blocks.as_mut() {
            for statement in basic_block.statements.iter_mut() {
