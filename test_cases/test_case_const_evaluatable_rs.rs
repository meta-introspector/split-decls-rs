// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs
// Error: expected square brackets
// Problematic line: line 24

use super::EvaluateConstErr;
use crate::traits::ObligationCtxt;

/// Check if a given constant can be evaluated.
#[instrument(skip(infcx), level = "debug")]
pub fn is_const_evaluatable<'tcx>(
    infcx: &InferCtxt<'tcx>,
