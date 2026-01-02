// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_public_bridge/src/builder.rs
// Error: expected square brackets
// Problematic line: line 11

use rustc_middle::mir::visit::MutVisitor;
use rustc_middle::ty::{self, TyCtxt};

/// Builds a monomorphic body for a given instance.
pub(crate) struct BodyBuilder<'tcx> {
    tcx: TyCtxt<'tcx>,
    instance: ty::Instance<'tcx>,
