// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_next_trait_solver/src/solve/effect_goals.rs
// Error: expected square brackets
// Problematic line: line 14


use super::assembly::{Candidate, structural_traits};
use crate::delegate::SolverDelegate;
use crate::solve::{
    BuiltinImplSource, CandidateSource, Certainty, EvalCtxt, Goal, GoalSource, NoSolution,
    QueryResult, assembly,
};
