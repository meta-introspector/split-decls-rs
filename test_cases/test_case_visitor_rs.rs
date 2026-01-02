// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/visitor.rs
// Error: expected square brackets
// Problematic line: line 14


use super::{InterpCx, MPlaceTy, Machine, Projectable, interp_ok, throw_inval};

/// How to traverse a value and what to do when we are at the leaves.
pub trait ValueVisitor<'tcx, M: Machine<'tcx>>: Sized {
    type V: Projectable<'tcx, M::Provenance> + From<MPlaceTy<'tcx, M::Provenance>>;

