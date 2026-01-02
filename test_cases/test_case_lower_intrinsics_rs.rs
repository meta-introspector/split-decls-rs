// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/lower_intrinsics.rs
// Error: expected square brackets
// Problematic line: line 12


pub(super) struct LowerIntrinsics;

impl<'tcx> crate::MirPass<'tcx> for LowerIntrinsics {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        let local_decls = &body.local_decls;
        for block in body.basic_blocks.as_mut() {
