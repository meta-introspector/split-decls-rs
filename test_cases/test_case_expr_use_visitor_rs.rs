// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/expr_use_visitor.rs
// Error: expected square brackets
// Problematic line: line 26

// Export these here so that Clippy can use them.
pub use rustc_middle::hir::place::{Place, PlaceBase, PlaceWithHirId, Projection};
use rustc_middle::mir::FakeReadCause;
use rustc_middle::ty::{
    self, BorrowKind, Ty, TyCtxt, TypeFoldable, TypeVisitableExt as _, adjustment,
};
use rustc_middle::{bug, span_bug};
