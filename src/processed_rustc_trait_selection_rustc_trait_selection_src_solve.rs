// SRC: ../rust/compiler/rustc_trait_selection/src/solve.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */
pub use rustc_next_trait_solver::solve::*;


pub(crate) use delegate::SolverDelegate;
pub use fulfill::{FulfillmentCtxt, NextSolverError, StalledOnCoroutines};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
pub(crate) use normalize::deeply_normalize_for_diagnostics;
pub use normalize::{
    deeply_normalize, deeply_normalize_with_skipped_universes,
    deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals,
};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=evaluate_root_goal_for_proof_tree_raw | COMPLEXITY=2 | LINES=13 */
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::ty::TyCtxt;
pub use select::InferCtxtSelectExt;

fn evaluate_root_goal_for_proof_tree_raw<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonical_input: CanonicalInput<TyCtxt<'tcx>>,
) -> (QueryResult<TyCtxt<'tcx>>, &'tcx inspect::Probe<TyCtxt<'tcx>>) {
    evaluate_root_goal_for_proof_tree_raw_provider::<SolverDelegate<'tcx>, TyCtxt<'tcx>>(
        tcx,
        canonical_input,
    )
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=provide | COMPLEXITY=3 | LINES=4 */

pub fn provide(providers: &mut Providers) {
    *providers = Providers { evaluate_root_goal_for_proof_tree_raw, ..*providers };
}