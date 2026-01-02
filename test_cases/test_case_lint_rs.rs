// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/lint.rs
// Error: expected square brackets
// Problematic line: line 15

use rustc_mir_dataflow::impls::{MaybeStorageDead, MaybeStorageLive, always_storage_live_locals};
use rustc_mir_dataflow::{Analysis, ResultsCursor};

pub(super) fn lint_body<'tcx>(tcx: TyCtxt<'tcx>, body: &Body<'tcx>, when: String) {
    let always_live_locals = &always_storage_live_locals(body);

    let maybe_storage_live = MaybeStorageLive::new(Cow::Borrowed(always_live_locals))
