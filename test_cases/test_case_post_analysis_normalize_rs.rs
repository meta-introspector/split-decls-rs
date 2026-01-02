// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/post_analysis_normalize.rs
// Error: expected square brackets
// Problematic line: line 11


pub(super) struct PostAnalysisNormalize;

impl<'tcx> crate::MirPass<'tcx> for PostAnalysisNormalize {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        // FIXME(#132279): This is used during the phase transition from analysis
        // to runtime, so we have to manually specify the correct typing mode.
