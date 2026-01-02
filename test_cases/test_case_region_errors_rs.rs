// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/diagnostics/region_errors.rs
// Error: expected square brackets
// Problematic line: line 17

use rustc_middle::bug;
use rustc_middle::hir::place::PlaceBase;
use rustc_middle::mir::{AnnotationSource, ConstraintCategory, ReturnConstraint};
use rustc_middle::ty::{
    self, GenericArgs, Region, RegionVid, Ty, TyCtxt, TypeFoldable, TypeVisitor, fold_regions,
};
use rustc_span::{Ident, Span, kw};
