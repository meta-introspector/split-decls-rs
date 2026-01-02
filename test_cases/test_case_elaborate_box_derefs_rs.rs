// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/elaborate_box_derefs.rs
// Error: expected square brackets
// Problematic line: line 14


use crate::patch::MirPatch;

/// Constructs the types used when accessing a Box's pointer
fn build_ptr_tys<'tcx>(
    tcx: TyCtxt<'tcx>,
    pointee: Ty<'tcx>,
