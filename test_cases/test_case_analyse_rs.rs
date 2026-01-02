// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/solve/inspect/analyse.rs
// Error: expected square brackets
// Problematic line: line 30

use crate::solve::delegate::SolverDelegate;
use crate::traits::ObligationCtxt;

pub struct InspectConfig {
    pub max_depth: usize,
}

