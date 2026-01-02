// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/transform.rs
// Error: expected square brackets
// Problematic line: line 12

use rustc_hir as hir;
use rustc_hir::LangItem;
use rustc_middle::bug;
use rustc_middle::ty::{
    self, ExistentialPredicateStableCmpExt as _, Instance, InstanceKind, IntTy, List, TraitRef, Ty,
    TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable, TypeVisitableExt, UintTy,
};
