// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/impl_wf_check/min_specialization.rs
// Error: expected square brackets
// Problematic line: line 74

use rustc_infer::traits::ObligationCause;
use rustc_infer::traits::specialization_graph::Node;
use rustc_middle::ty::trait_def::TraitSpecializationKind;
use rustc_middle::ty::{
    self, GenericArg, GenericArgs, GenericArgsRef, TyCtxt, TypeVisitableExt, TypingMode,
};
use rustc_span::{ErrorGuaranteed, Span};
