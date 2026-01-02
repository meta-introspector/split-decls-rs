// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ty_utils/src/needs_drop.rs
// Error: expected square brackets
// Problematic line: line 17


type NeedsDropResult<T> = Result<T, AlwaysRequiresDrop>;

fn needs_drop_raw<'tcx>(
    tcx: TyCtxt<'tcx>,
    query: ty::PseudoCanonicalInput<'tcx, Ty<'tcx>>,
) -> bool {
