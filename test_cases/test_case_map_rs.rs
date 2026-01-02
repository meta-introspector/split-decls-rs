// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/hir/map.rs
// Error: expected square brackets
// Problematic line: line 26

use crate::query::LocalCrate;
use crate::ty::TyCtxt;

/// An iterator that walks up the ancestor tree of a given `HirId`.
/// Constructed using `tcx.hir_parent_iter(hir_id)`.
struct ParentHirIterator<'tcx> {
    current_id: HirId,
