// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/structural_impls.rs
// Error: expected square brackets
// Problematic line: line 18

use super::{GenericArg, GenericArgKind, Pattern, Region};
use crate::mir::PlaceElem;
use crate::ty::print::{FmtPrinter, Printer, with_no_trimmed_paths};
use crate::ty::{
    self, FallibleTypeFolder, Lift, Term, TermKind, Ty, TyCtxt, TypeFoldable, TypeSuperFoldable,
    TypeSuperVisitable, TypeVisitable, TypeVisitor,
};
