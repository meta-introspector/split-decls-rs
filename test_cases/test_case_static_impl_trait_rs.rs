// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/static_impl_trait.rs
// Error: expected square brackets
// Problematic line: line 7

use rustc_errors::{Applicability, Diag, ErrorGuaranteed};
use rustc_hir::def_id::DefId;
use rustc_hir::intravisit::{Visitor, VisitorExt, walk_ty};
use rustc_hir::{
    self as hir, AmbigArg, GenericBound, GenericParam, GenericParamKind, Item, ItemKind, Lifetime,
    LifetimeKind, LifetimeParamKind, MissingLifetimeKind, Node, TyKind,
};
