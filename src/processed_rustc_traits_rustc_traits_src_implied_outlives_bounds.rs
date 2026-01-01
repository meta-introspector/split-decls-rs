// SRC: ../rust/compiler/rustc_traits/src/implied_outlives_bounds.rs
// Provider for the `implied_outlives_bounds` query.
// Do not call this query directory. See
// [`crate::rustc_trait_selection::traits::query::type_op::implied_outlives_bounds`].

use crate::rustc_infer::infer::TyCtxtInferExt;
use crate::rustc_infer::infer::canonical::{self, Canonical};
use crate::rustc_infer::traits::query::OutlivesBound;
use crate::rustc_infer::traits::query::type_op::ImpliedOutlivesBounds;
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::ty::{ParamEnvAnd, TyCtxt};
use crate::rustc_complete::DUMMY_SP;
use crate::rustc_trait_selection::infer::InferCtxtBuilderExt;
use crate::rustc_trait_selection::traits::query::type_op::implied_outlives_bounds::compute_implied_outlives_bounds_inner;
use crate::rustc_trait_selection::traits::query::{CanonicalImpliedOutlivesBoundsGoal, NoSolution};

pub(crate) fn provide(p: &mut Providers) {
    *p = Providers { implied_outlives_bounds, ..*p };
}

fn implied_outlives_bounds<'tcx>(
    tcx: TyCtxt<'tcx>,
    (goal, disable_implied_bounds_hack): (CanonicalImpliedOutlivesBoundsGoal<'tcx>, bool),
) -> Result<
    &'tcx Canonical<'tcx, canonical::QueryResponse<'tcx, Vec<OutlivesBound<'tcx>>>>,
    NoSolution,
> {
    tcx.infer_ctxt().enter_canonical_trait_query(&goal, |ocx, key| {
        let ParamEnvAnd { param_env, value: ImpliedOutlivesBounds { ty } } = key;
        compute_implied_outlives_bounds_inner(
            ocx,
            param_env,
            ty,
            DUMMY_SP,
            disable_implied_bounds_hack,
        )
    })
}