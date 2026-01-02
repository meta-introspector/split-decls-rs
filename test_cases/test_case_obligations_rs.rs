// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/outlives/obligations.rs
// Error: expected square brackets
// Problematic line: line 67

use rustc_middle::mir::ConstraintCategory;
use rustc_middle::traits::query::NoSolution;
use rustc_middle::ty::outlives::{Component, push_outlives_components};
use rustc_middle::ty::{
    self, GenericArgKind, GenericArgsRef, PolyTypeOutlivesPredicate, Region, Ty, TyCtxt,
    TypeFoldable as _, TypeVisitableExt,
};
