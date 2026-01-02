// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/add_retag.rs
// Error: expected square brackets
// Problematic line: line 12


pub(super) struct AddRetag;

/// Determine whether this type may contain a reference (or box), and thus needs retagging.
/// We will only recurse `depth` times into Tuples/ADTs to bound the cost of this.
fn may_contain_reference<'tcx>(ty: Ty<'tcx>, depth: u32, tcx: TyCtxt<'tcx>) -> bool {
    match ty.kind() {
