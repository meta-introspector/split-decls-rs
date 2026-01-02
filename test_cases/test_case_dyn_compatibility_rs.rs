// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/dyn_compatibility.rs
// Error: expected square brackets
// Problematic line: line 13

use rustc_hir::def_id::DefId;
use rustc_hir::{self as hir, LangItem};
use rustc_middle::query::Providers;
use rustc_middle::ty::{
    self, EarlyBinder, GenericArgs, Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable,
    TypeSuperVisitable, TypeVisitable, TypeVisitableExt, TypeVisitor, TypingMode, Upcast,
    elaborate,
