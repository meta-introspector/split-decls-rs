// SRC: ../rust/compiler/rustc_traits/src/dropck_outlives.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_data_structures::fx::FxHashSet;
use crate::rustc_infer::infer::TyCtxtInferExt;
use crate::rustc_infer::infer::canonical::{Canonical, QueryResponse};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_complete::bug;
use crate::rustc_complete::query::Providers;
use crate::rustc_complete::traits::query::{DropckConstraint, DropckOutlivesResult};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::ty::{self, GenericArgs, TyCtxt};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */
use crate::rustc_complete::DUMMY_SP;
use crate::rustc_complete::def_id::DefId;
use crate::rustc_trait_selection::infer::InferCtxtBuilderExt;
use crate::rustc_trait_selection::traits::query::dropck_outlives::{
    compute_dropck_outlives_inner, dtorck_constraint_for_ty_inner,
};
/* AST_META: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_trait_selection::traits::query::{CanonicalDropckOutlivesGoal, NoSolution};
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=5 */
use tracing::debug;

pub(crate) fn provide(p: &mut Providers) {
    *p = Providers { dropck_outlives, adt_dtorck_constraint, ..*p };
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=dropck_outlives | COMPLEXITY=4 | LINES=11 */

fn dropck_outlives<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonical_goal: CanonicalDropckOutlivesGoal<'tcx>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, DropckOutlivesResult<'tcx>>>, NoSolution> {
    debug!("dropck_outlives(goal={:#?})", canonical_goal);

    tcx.infer_ctxt().enter_canonical_trait_query(&canonical_goal, |ocx, goal| {
        compute_dropck_outlives_inner(ocx, goal, DUMMY_SP)
    })
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=21 | LINES=36 */

/// Calculates the dtorck constraint for a type.
pub(crate) fn adt_dtorck_constraint(tcx: TyCtxt<'_>, def_id: DefId) -> &DropckConstraint<'_> {
    let def = tcx.adt_def(def_id);
    let span = tcx.def_span(def_id);
    let typing_env = ty::TypingEnv::non_body_analysis(tcx, def_id);
    debug!("dtorck_constraint: {:?}", def);

    if def.is_manually_drop() {
        bug!("`ManuallyDrop` should have been handled by `trivial_dropck_outlives`");
    } else if def.is_phantom_data() {
        // The first generic parameter here is guaranteed to be a type because it's
        // `PhantomData`.
        let args = GenericArgs::identity_for_item(tcx, def_id);
        assert_eq!(args.len(), 1);
        let result = DropckConstraint {
            outlives: vec![],
            dtorck_types: vec![args.type_at(0)],
            overflows: vec![],
        };
        debug!("dtorck_constraint: {:?} => {:?}", def, result);
        return tcx.arena.alloc(result);
    }

    let mut result = DropckConstraint::empty();
    for field in def.all_fields() {
        let fty = tcx.type_of(field.did).instantiate_identity();
        dtorck_constraint_for_ty_inner(tcx, typing_env, span, 0, fty, &mut result);
    }
    result.outlives.extend(tcx.destructor_constraints(def));
    dedup_dtorck_constraint(&mut result);

    debug!("dtorck_constraint: {:?} => {:?}", def, result);

    tcx.arena.alloc(result)
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=dedup_dtorck_constraint | COMPLEXITY=2 | LINES=8 */

fn dedup_dtorck_constraint(c: &mut DropckConstraint<'_>) {
    let mut outlives = FxHashSet::default();
    let mut dtorck_types = FxHashSet::default();

    c.outlives.retain(|&val| outlives.replace(val).is_none());
    c.dtorck_types.retain(|&val| dtorck_types.replace(val).is_none());
}