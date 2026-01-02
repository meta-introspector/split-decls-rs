// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/check_inline.rs
// Error: expected square brackets
// Problematic line: line 16


pub(super) struct CheckForceInline;

impl<'tcx> MirLint<'tcx> for CheckForceInline {
    fn run_lint(&self, tcx: TyCtxt<'tcx>, body: &Body<'tcx>) {
        let def_id = body.source.def_id();
        if !tcx.hir_body_owner_kind(def_id).is_fn_or_closure() || !def_id.is_local() {
