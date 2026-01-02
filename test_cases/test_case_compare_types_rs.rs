// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/util/compare_types.rs
// Error: expected square brackets
// Problematic line: line 11

use rustc_middle::ty::{Ty, TyCtxt, TypingEnv, Variance};
use rustc_trait_selection::traits::ObligationCtxt;

/// Returns whether `src` is a subtype of `dest`, i.e. `src <: dest`.
pub fn sub_types<'tcx>(
    tcx: TyCtxt<'tcx>,
    typing_env: TypingEnv<'tcx>,
