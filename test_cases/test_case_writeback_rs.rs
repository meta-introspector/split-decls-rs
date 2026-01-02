// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/writeback.rs
// Error: expected square brackets
// Problematic line: line 23

use rustc_infer::traits::solve::Goal;
use rustc_middle::traits::ObligationCause;
use rustc_middle::ty::adjustment::{Adjust, Adjustment, PointerCoercion};
use rustc_middle::ty::{
    self, DefiningScopeKind, OpaqueHiddenType, Ty, TyCtxt, TypeFoldable, TypeFolder,
    TypeSuperFoldable, TypeSuperVisitable, TypeVisitable, TypeVisitableExt, TypeVisitor,
    fold_regions,
