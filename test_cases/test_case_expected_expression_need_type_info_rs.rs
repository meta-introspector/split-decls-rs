// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/need_type_info.rs
// Error: expected an expression
// Error type: expected_expression
// Sample #3 of 3
// Problematic line: line 16

use rustc_middle::hir::nested_filter;
use rustc_middle::ty::adjustment::{Adjust, Adjustment, AutoBorrow};
use rustc_middle::ty::print::{FmtPrinter, PrettyPrinter, Print, Printer};
use rustc_middle::ty::{
    self, GenericArg, GenericArgKind, GenericArgsRef, InferConst, IsSuggestable, Term, TermKind,
    Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable, TypeVisitableExt, TypeckResults,
};
