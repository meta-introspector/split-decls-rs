// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/stability.rs
// Error: expected square brackets
// Problematic line: line 14

use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{CRATE_DEF_ID, LOCAL_CRATE, LocalDefId, LocalModDefId};
use rustc_hir::intravisit::{self, Visitor, VisitorExt};
use rustc_hir::{
    self as hir, AmbigArg, ConstStability, DefaultBodyStability, FieldDef, Item, ItemKind,
    Stability, StabilityLevel, StableSince, TraitRef, Ty, TyKind, UnstableReason,
    VERSION_PLACEHOLDER, Variant, find_attr,
