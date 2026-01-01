// SRC: ../rust/compiler/rustc_traits/src/evaluate_obligation.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_infer::infer::TyCtxtInferExt;
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::ty::{ParamEnvAnd, TyCtxt};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */
use crate::rustc_complete::DUMMY_SP;
use crate::rustc_trait_selection::traits::query::CanonicalPredicateGoal;
use crate::rustc_trait_selection::traits::{
    EvaluationResult, Obligation, ObligationCause, OverflowError, SelectionContext, TraitQueryMode,
    sizedness_fast_path,
};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=5 */
use tracing::debug;

pub(crate) fn provide(p: &mut Providers) {
    *p = Providers { evaluate_obligation, ..*p };
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=evaluate_obligation | COMPLEXITY=9 | LINES=21 */

fn evaluate_obligation<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonical_goal: CanonicalPredicateGoal<'tcx>,
) -> Result<EvaluationResult, OverflowError> {
    assert!(!tcx.next_trait_solver_globally());
    debug!("evaluate_obligation(canonical_goal={:#?})", canonical_goal);
    let (ref infcx, goal, _var_values) =
        tcx.infer_ctxt().build_with_canonical(DUMMY_SP, &canonical_goal);
    debug!("evaluate_obligation: goal={:#?}", goal);
    let ParamEnvAnd { param_env, value: predicate } = goal;

    if sizedness_fast_path(tcx, predicate, param_env) {
        return Ok(EvaluationResult::EvaluatedToOk);
    }

    let mut selcx = SelectionContext::with_query_mode(infcx, TraitQueryMode::Canonical);
    let obligation = Obligation::new(tcx, ObligationCause::dummy(), param_env, predicate);

    selcx.evaluate_root_obligation(&obligation)
}