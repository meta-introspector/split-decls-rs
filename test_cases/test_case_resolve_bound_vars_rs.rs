// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/collect/resolve_bound_vars.rs
// Error: expected square brackets
// Problematic line: line 19

use rustc_hir::def::{DefKind, Res};
use rustc_hir::definitions::{DefPathData, DisambiguatorState};
use rustc_hir::intravisit::{self, InferKind, Visitor, VisitorExt};
use rustc_hir::{
    self as hir, AmbigArg, GenericArg, GenericParam, GenericParamKind, HirId, LifetimeKind, Node,
};
use rustc_macros::extension;
