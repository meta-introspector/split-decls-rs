// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/upvar.rs
// Error: expected square brackets
// Problematic line: line 46

use rustc_middle::hir::place::{Place, PlaceBase, PlaceWithHirId, Projection, ProjectionKind};
use rustc_middle::mir::FakeReadCause;
use rustc_middle::traits::ObligationCauseCode;
use rustc_middle::ty::{
    self, BorrowKind, ClosureSizeProfileData, Ty, TyCtxt, TypeVisitableExt as _, TypeckResults,
    UpvarArgs, UpvarCapture,
};
