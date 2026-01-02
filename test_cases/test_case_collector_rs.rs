// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_monomorphize/src/collector.rs
// Error: expected square brackets
// Problematic line: line 229

use rustc_middle::query::TyCtxtAt;
use rustc_middle::ty::adjustment::{CustomCoerceUnsized, PointerCoercion};
use rustc_middle::ty::layout::ValidityRequirement;
use rustc_middle::ty::{
    self, GenericArgs, GenericParamDefKind, Instance, InstanceKind, Ty, TyCtxt, TypeFoldable,
    TypeVisitableExt, VtblEntry,
};
