// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/inspect_obligations.rs
// Error: expected square brackets
// Problematic line: line 8

use rustc_middle::ty::{self, Ty, TypeVisitableExt};
use rustc_span::Span;
use rustc_trait_selection::solve::Certainty;
use rustc_trait_selection::solve::inspect::{
    InspectConfig, InspectGoal, ProofTreeInferCtxtExt, ProofTreeVisitor,
};
use tracing::{debug, instrument, trace};
