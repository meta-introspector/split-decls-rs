// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/diagnostic_items.rs
// Error: expected square brackets
// Problematic line: line 21


use crate::errors::DuplicateDiagnosticItemInCrate;

fn observe_item<'tcx>(tcx: TyCtxt<'tcx>, diagnostic_items: &mut DiagnosticItems, owner: OwnerId) {
    let attrs = tcx.hir_attrs(owner.into());
    if let Some(name) = extract(attrs) {
        // insert into our table
