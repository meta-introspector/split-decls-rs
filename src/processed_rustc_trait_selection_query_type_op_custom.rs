// SRC: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/custom.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */
use std::fmt;

use crate::rustc_complete::ErrorGuaranteed;
use crate::rustc_complete::def_id::LocalDefId;
use crate::rustc_infer::infer::region_constraints::RegionConstraintData;
use crate::rustc_complete::traits::query::NoSolution;
use crate::rustc_complete::ty::{TyCtxt, TypeFoldable};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=CustomTypeOp | COMPLEXITY=2 | LINES=12 */
use crate::rustc_complete::Span;
use tracing::info;

use crate::infer::InferCtxt;
use crate::infer::canonical::query_response;
use crate::traits::ObligationCtxt;
use crate::traits::query::type_op::TypeOpOutput;

pub struct CustomTypeOp<F> {
    closure: F,
    description: &'static str,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=9 */

impl<F> CustomTypeOp<F> {
    pub fn new<'tcx, R>(closure: F, description: &'static str) -> Self
    where
        F: FnOnce(&ObligationCtxt<'_, 'tcx>) -> Result<R, NoSolution>,
    {
        CustomTypeOp { closure, description }
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=fully_perform | COMPLEXITY=12 | LINES=27 */

impl<'tcx, F, R> super::TypeOp<'tcx> for CustomTypeOp<F>
where
    F: FnOnce(&ObligationCtxt<'_, 'tcx>) -> Result<R, NoSolution>,
    R: fmt::Debug + TypeFoldable<TyCtxt<'tcx>>,
{
    type Output = R;
    /// We can't do any custom error reporting for `CustomTypeOp`, so
    /// we can use `!` to enforce that the implementation never provides it.
    type ErrorInfo = !;

    /// Processes the operation and all resulting obligations,
    /// returning the final result along with any region constraints
    /// (they will be given over to the NLL region solver).
    fn fully_perform(
        self,
        infcx: &InferCtxt<'tcx>,
        root_def_id: LocalDefId,
        span: Span,
    ) -> Result<TypeOpOutput<'tcx, Self>, ErrorGuaranteed> {
        if cfg!(debug_assertions) {
            info!("fully_perform({:?})", self);
        }

        Ok(scrape_region_constraints(infcx, root_def_id, self.description, span, self.closure)?.0)
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6 */

impl<F> fmt::Debug for CustomTypeOp<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.description.fmt(f)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=scrape_region_constraints | COMPLEXITY=26 | LINES=75 */

/// Executes `op` and then scrapes out all the "old style" region
/// constraints that result, creating query-region-constraints.
pub fn scrape_region_constraints<'tcx, Op, R>(
    infcx: &InferCtxt<'tcx>,
    root_def_id: LocalDefId,
    name: &'static str,
    span: Span,
    op: impl FnOnce(&ObligationCtxt<'_, 'tcx>) -> Result<R, NoSolution>,
) -> Result<(TypeOpOutput<'tcx, Op>, RegionConstraintData<'tcx>), ErrorGuaranteed>
where
    R: TypeFoldable<TyCtxt<'tcx>>,
    Op: super::TypeOp<'tcx, Output = R>,
{
    // During NLL, we expect that nobody will register region
    // obligations **except** as part of a custom type op (and, at the
    // end of each custom type op, we scrape out the region
    // obligations that resulted). So this vector should be empty on
    // entry.
    let pre_obligations = infcx.take_registered_region_obligations();
    assert!(
        pre_obligations.is_empty(),
        "scrape_region_constraints: incoming region obligations = {pre_obligations:#?}",
    );
    let pre_assumptions = infcx.take_registered_region_assumptions();
    assert!(
        pre_assumptions.is_empty(),
        "scrape_region_constraints: incoming region assumptions = {pre_assumptions:#?}",
    );

    let value = infcx.commit_if_ok(|_| {
        let ocx = ObligationCtxt::new(infcx);
        let value = op(&ocx).map_err(|_| {
            infcx.dcx().span_delayed_bug(span, format!("error performing operation: {name}"))
        })?;
        let errors = ocx.select_all_or_error();
        if errors.is_empty() {
            Ok(value)
        } else if let Err(guar) = infcx.tcx.check_potentially_region_dependent_goals(root_def_id) {
            Err(guar)
        } else {
            Err(infcx
                .dcx()
                .delayed_bug(format!("errors selecting obligation during MIR typeck: {errors:?}")))
        }
    })?;

    // Next trait solver performs operations locally, and normalize goals should resolve vars.
    let value = infcx.resolve_vars_if_possible(value);

    let region_obligations = infcx.take_registered_region_obligations();
    let region_assumptions = infcx.take_registered_region_assumptions();
    let region_constraint_data = infcx.take_and_reset_region_constraints();
    let region_constraints = query_response::make_query_region_constraints(
        region_obligations,
        &region_constraint_data,
        region_assumptions,
    );

    if region_constraints.is_empty() {
        Ok((
            TypeOpOutput { output: value, constraints: None, error_info: None },
            region_constraint_data,
        ))
    } else {
        Ok((
            TypeOpOutput {
                output: value,
                constraints: Some(infcx.tcx.arena.alloc(region_constraints)),
                error_info: None,
            },
            region_constraint_data,
        ))
    }
}