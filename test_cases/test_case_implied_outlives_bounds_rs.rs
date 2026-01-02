// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_traits/src/implied_outlives_bounds.rs
// Error: expected square brackets
// Problematic line: line 16

use rustc_trait_selection::traits::query::type_op::implied_outlives_bounds::compute_implied_outlives_bounds_inner;
use rustc_trait_selection::traits::query::{CanonicalImpliedOutlivesBoundsGoal, NoSolution};

pub(crate) fn provide(p: &mut Providers) {
    *p = Providers { implied_outlives_bounds, ..*p };
}

