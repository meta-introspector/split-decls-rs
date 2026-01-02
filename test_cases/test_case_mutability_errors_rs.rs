// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs
// Error: expected square brackets
// Problematic line: line 15

use rustc_middle::bug;
use rustc_middle::hir::place::PlaceBase;
use rustc_middle::mir::visit::PlaceContext;
use rustc_middle::mir::{
    self, BindingForm, Body, BorrowKind, Local, LocalDecl, LocalInfo, LocalKind, Location,
    Mutability, Operand, Place, PlaceRef, ProjectionElem, RawPtrKind, Rvalue, Statement,
    StatementKind, TerminatorKind,
