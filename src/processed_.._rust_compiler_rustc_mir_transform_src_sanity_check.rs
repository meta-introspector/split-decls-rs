// SRC: ../rust/compiler/rustc_mir_transform/src/sanity_check.rs
use rustc_middle::mir::Body;
use rustc_middle::ty::TyCtxt;
use rustc_mir_dataflow::rustc_peek::sanity_check;

pub(super) struct SanityCheck;

impl<'tcx> crate::MirLint<'tcx> for SanityCheck {
    fn run_lint(&self, tcx: TyCtxt<'tcx>, body: &Body<'tcx>) {
        sanity_check(tcx, body);
    }
}
