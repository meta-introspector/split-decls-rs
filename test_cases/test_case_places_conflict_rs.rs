// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/places_conflict.rs
// Error: expected square brackets
// Problematic line: line 58


use rustc_hir as hir;
use rustc_middle::bug;
use rustc_middle::mir::{
    Body, BorrowKind, FakeBorrowKind, MutBorrowKind, Place, PlaceElem, PlaceRef, ProjectionElem,
};
use rustc_middle::ty::{self, TyCtxt};
