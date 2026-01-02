// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/coherence.rs
// Error: expected square brackets
// Problematic line: line 21

use rustc_middle::traits::solve::{CandidateSource, Certainty, Goal};
use rustc_middle::traits::specialization_graph::OverlapMode;
use rustc_middle::ty::fast_reject::DeepRejectCtxt;
use rustc_middle::ty::{
    self, Ty, TyCtxt, TypeSuperVisitable, TypeVisitable, TypeVisitableExt, TypeVisitor, TypingMode,
};
pub use rustc_next_trait_solver::coherence::*;
