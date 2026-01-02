// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_public_bridge/src/context/impls.rs
// Error: expected square brackets
// Problematic line: line 15

use rustc_middle::ty::layout::{FnAbiOf, LayoutOf};
use rustc_middle::ty::print::{with_forced_trimmed_paths, with_no_trimmed_paths};
use rustc_middle::ty::util::Discr;
use rustc_middle::ty::{
    AdtDef, AdtKind, AssocItem, Binder, ClosureKind, CoroutineArgsExt, EarlyBinder,
    ExistentialTraitRef, FnSig, GenericArgsRef, Instance, InstanceKind, IntrinsicDef, List,
    PolyFnSig, ScalarInt, TraitDef, TraitRef, Ty, TyCtxt, TyKind, TypeVisitableExt, UintTy,
