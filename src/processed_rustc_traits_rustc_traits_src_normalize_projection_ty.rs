// SRC: ../rust/compiler/rustc_traits/src/normalize_projection_ty.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_infer::infer::TyCtxtInferExt;
use crate::rustc_infer::infer::canonical::{Canonical, QueryResponse};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_infer::traits::PredicateObligations;
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::ty::{ParamEnvAnd, TyCtxt};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use crate::rustc_trait_selection::error_reporting::InferCtxtErrorExt;
use crate::rustc_trait_selection::infer::InferCtxtBuilderExt;
use crate::rustc_trait_selection::traits::query::normalize::NormalizationResult;
use crate::rustc_trait_selection::traits::query::{CanonicalAliasGoal, NoSolution};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_trait_selection::traits::{self, ObligationCause, ScrubbedTraitError, SelectionContext};
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=10 */
use tracing::debug;

pub(crate) fn provide(p: &mut Providers) {
    *p = Providers {
        normalize_canonicalized_projection_ty,
        normalize_canonicalized_free_alias,
        normalize_canonicalized_inherent_projection_ty,
        ..*p
    };
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=normalize_canonicalized_projection_ty | COMPLEXITY=24 | LINES=50 */

fn normalize_canonicalized_projection_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    goal: CanonicalAliasGoal<'tcx>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, NormalizationResult<'tcx>>>, NoSolution> {
    debug!("normalize_canonicalized_projection_ty(goal={:#?})", goal);

    tcx.infer_ctxt().enter_canonical_trait_query(
        &goal,
        |ocx, ParamEnvAnd { param_env, value: goal }| {
            debug_assert!(!ocx.infcx.next_trait_solver());
            let selcx = &mut SelectionContext::new(ocx.infcx);
            let cause = ObligationCause::dummy();
            let mut obligations = PredicateObligations::new();
            let answer = traits::normalize_projection_term(
                selcx,
                param_env,
                goal.into(),
                cause,
                0,
                &mut obligations,
            );
            ocx.register_obligations(obligations);
            // #112047: With projections and opaques, we are able to create opaques that
            // are recursive (given some generic parameters of the opaque's type variables).
            // In that case, we may only realize a cycle error when calling
            // `normalize_erasing_regions` in mono.
            let errors = ocx.select_where_possible();
            if !errors.is_empty() {
                // Rustdoc may attempt to normalize type alias types which are not
                // well-formed. Rustdoc also normalizes types that are just not
                // well-formed, since we don't do as much HIR analysis (checking
                // that impl vars are constrained by the signature, for example).
                if !tcx.sess.opts.actually_rustdoc {
                    for error in &errors {
                        if let ScrubbedTraitError::Cycle(cycle) = &error {
                            ocx.infcx.err_ctxt().report_overflow_obligation_cycle(cycle);
                        }
                    }
                }
                return Err(NoSolution);
            }

            // FIXME(associated_const_equality): All users of normalize_canonicalized_projection_ty
            // expected a type, but there is the possibility it could've been a const now.
            // Maybe change it to a Term later?
            Ok(NormalizationResult { normalized_ty: answer.expect_type() })
        },
    )
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=normalize_canonicalized_free_alias | COMPLEXITY=9 | LINES=26 */

fn normalize_canonicalized_free_alias<'tcx>(
    tcx: TyCtxt<'tcx>,
    goal: CanonicalAliasGoal<'tcx>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, NormalizationResult<'tcx>>>, NoSolution> {
    debug!("normalize_canonicalized_free_alias(goal={:#?})", goal);

    tcx.infer_ctxt().enter_canonical_trait_query(
        &goal,
        |ocx, ParamEnvAnd { param_env, value: goal }| {
            let obligations = tcx.predicates_of(goal.def_id).instantiate_own(tcx, goal.args).map(
                |(predicate, span)| {
                    traits::Obligation::new(
                        tcx,
                        ObligationCause::dummy_with_span(span),
                        param_env,
                        predicate,
                    )
                },
            );
            ocx.register_obligations(obligations);
            let normalized_ty = tcx.type_of(goal.def_id).instantiate(tcx, goal.args);
            Ok(NormalizationResult { normalized_ty })
        },
    )
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=normalize_canonicalized_inherent_projection_ty | COMPLEXITY=7 | LINES=27 */

fn normalize_canonicalized_inherent_projection_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    goal: CanonicalAliasGoal<'tcx>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, NormalizationResult<'tcx>>>, NoSolution> {
    debug!("normalize_canonicalized_inherent_projection_ty(goal={:#?})", goal);

    tcx.infer_ctxt().enter_canonical_trait_query(
        &goal,
        |ocx, ParamEnvAnd { param_env, value: goal }| {
            let selcx = &mut SelectionContext::new(ocx.infcx);
            let cause = ObligationCause::dummy();
            let mut obligations = PredicateObligations::new();
            let answer = traits::normalize_inherent_projection(
                selcx,
                param_env,
                goal.into(),
                cause,
                0,
                &mut obligations,
            );
            ocx.register_obligations(obligations);

            Ok(NormalizationResult { normalized_ty: answer.expect_type() })
        },
    )
}