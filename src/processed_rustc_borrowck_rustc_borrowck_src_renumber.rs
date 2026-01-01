// SRC: ../rust/compiler/rustc_borrowck/src/renumber.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use crate::rustc_index::IndexSlice;
use crate::rustc_infer::infer::NllRegionVariableOrigin;
use crate::rustc_complete::mir::visit::{MutVisitor, TyContext};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::mir::{Body, ConstOperand, Location, Promoted};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::ty::{self, GenericArgsRef, Ty, TyCtxt, TypeFoldable, fold_regions};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::Symbol;
use tracing::{debug, instrument};
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=21 */

use crate::BorrowckInferCtxt;

/// Replaces all free regions appearing in the MIR with fresh
/// inference variables, returning the number of variables created.
#[instrument(skip(infcx, body, promoted), level = "debug")]
pub(crate) fn renumber_mir<'tcx>(
    infcx: &BorrowckInferCtxt<'tcx>,
    body: &mut Body<'tcx>,
    promoted: &mut IndexSlice<Promoted, Body<'tcx>>,
) {
    debug!(?body.arg_count);

    let mut renumberer = RegionRenumberer { infcx };

    for body in promoted.iter_mut() {
        renumberer.visit_body(body);
    }

    renumberer.visit_body(body);
}
/* AST_META: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=12 */

// The fields are used only for debugging output in `sccs_info`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum RegionCtxt {
    Location(Location),
    TyContext(TyContext),
    Free(Symbol),
    LateBound(Symbol),
    Existential(Option<Symbol>),
    Placeholder(Symbol),
    Unknown,
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=15 */

impl RegionCtxt {
    /// Used to determine the representative of a component in the strongly connected
    /// constraint graph
    pub(crate) fn preference_value(self) -> usize {
        match self {
            RegionCtxt::Unknown => 1,
            RegionCtxt::Existential(None) => 2,
            RegionCtxt::Existential(Some(_)) | RegionCtxt::Free(_) => 2,
            RegionCtxt::Location(_) => 3,
            RegionCtxt::TyContext(_) => 4,
            _ => 5,
        }
    }
}
/* AST_META: AST_ID=8 | TYPE=STRUCT | NAME=RegionRenumberer | COMPLEXITY=2 | LINES=4 */

struct RegionRenumberer<'a, 'tcx> {
    infcx: &'a BorrowckInferCtxt<'tcx>,
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=renumber_regions | COMPLEXITY=6 | LINES=15 */

impl<'a, 'tcx> RegionRenumberer<'a, 'tcx> {
    /// Replaces all regions appearing in `value` with fresh inference
    /// variables.
    fn renumber_regions<T, F>(&mut self, value: T, region_ctxt_fn: F) -> T
    where
        T: TypeFoldable<TyCtxt<'tcx>>,
        F: Fn() -> RegionCtxt,
    {
        let origin = NllRegionVariableOrigin::Existential { name: None };
        fold_regions(self.infcx.tcx, value, |_region, _depth| {
            self.infcx.next_nll_region_var(origin, || region_ctxt_fn())
        })
    }
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=tcx | COMPLEXITY=17 | LINES=47 */

impl<'a, 'tcx> MutVisitor<'tcx> for RegionRenumberer<'a, 'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.infcx.tcx
    }

    #[instrument(skip(self), level = "debug")]
    fn visit_ty(&mut self, ty: &mut Ty<'tcx>, ty_context: TyContext) {
        if matches!(ty_context, TyContext::ReturnTy(_)) {
            // We will renumber the return ty when called again with `TyContext::LocalDecl`
            return;
        }
        *ty = self.renumber_regions(*ty, || RegionCtxt::TyContext(ty_context));

        debug!(?ty);
    }

    #[instrument(skip(self), level = "debug")]
    fn visit_args(&mut self, args: &mut GenericArgsRef<'tcx>, location: Location) {
        *args = self.renumber_regions(*args, || RegionCtxt::Location(location));

        debug!(?args);
    }

    #[instrument(skip(self), level = "debug")]
    fn visit_region(&mut self, region: &mut ty::Region<'tcx>, location: Location) {
        let old_region = *region;
        *region = self.renumber_regions(old_region, || RegionCtxt::Location(location));

        debug!(?region);
    }

    #[instrument(skip(self), level = "debug")]
    fn visit_ty_const(&mut self, ct: &mut ty::Const<'tcx>, location: Location) {
        let old_ct = *ct;
        *ct = self.renumber_regions(old_ct, || RegionCtxt::Location(location));

        debug!(?ct);
    }

    #[instrument(skip(self), level = "debug")]
    fn visit_const_operand(&mut self, constant: &mut ConstOperand<'tcx>, location: Location) {
        let const_ = constant.const_;
        constant.const_ = self.renumber_regions(const_, || RegionCtxt::Location(location));
        debug!("constant: {:#?}", constant);
    }
}