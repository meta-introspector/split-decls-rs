// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs
// Error: expected square brackets
// Problematic line: line 20

use rustc_hir::{CoroutineDesugaring, CoroutineKind, CoroutineSource, LangItem, PatField};
use rustc_middle::bug;
use rustc_middle::hir::nested_filter::OnlyBodies;
use rustc_middle::mir::{
    self, AggregateKind, BindingForm, BorrowKind, ClearCrossCrate, ConstraintCategory,
    FakeBorrowKind, FakeReadCause, LocalDecl, LocalInfo, LocalKind, Location, MutBorrowKind,
    Operand, Place, PlaceRef, PlaceTy, ProjectionElem, Rvalue, Statement, StatementKind,
