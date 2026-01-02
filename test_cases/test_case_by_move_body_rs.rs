// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/coroutine/by_move_body.rs
// Error: expected square brackets
// Problematic line: line 83

use rustc_middle::mir::{self, MirDumper};
use rustc_middle::ty::{self, InstanceKind, Ty, TyCtxt, TypeVisitableExt};

pub(crate) fn coroutine_by_move_body_def_id<'tcx>(
    tcx: TyCtxt<'tcx>,
    coroutine_def_id: LocalDefId,
) -> DefId {
