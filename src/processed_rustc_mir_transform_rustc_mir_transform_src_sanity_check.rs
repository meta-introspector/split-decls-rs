// SRC: ../rust/compiler/rustc_mir_transform/src/sanity_check.rs
use crate::rustc_complete::mir::Body;
use crate::rustc_complete::ty::TyCtxt;
use crate::rustc_mir_dataflow::rustc_peek::sanity_check;

pub(super) struct SanityCheck;

impl<'tcx> crate::MirLint<'tcx> for SanityCheck {
    fn run_lint(&self, tcx: TyCtxt<'tcx>, body: &Body<'tcx>) {
        sanity_check(tcx, body);
    }
}