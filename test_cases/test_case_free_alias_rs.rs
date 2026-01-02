// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/free_alias.rs
// Error: expected square brackets
// Problematic line: line 12

use crate::delegate::SolverDelegate;
use crate::solve::{Certainty, EvalCtxt, Goal, GoalSource, QueryResult};

impl<D, I> EvalCtxt<'_, D>
where
    D: SolverDelegate<Interner = I>,
    I: Interner,
