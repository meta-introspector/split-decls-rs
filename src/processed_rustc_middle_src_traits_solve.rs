// SRC: ../rust/compiler/rustc_middle/src/traits/solve.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */
use crate::rustc_data_structures::intern::Interned;
use rustc_macros::HashStable;
use rustc_type_ir as ir;
pub use rustc_type_ir::solve::*;

use crate::ty::{
    self, FallibleTypeFolder, TyCtxt, TypeFoldable, TypeFolder, TypeVisitable, TypeVisitor,
    try_visit,
};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=PredefinedOpaques | COMPLEXITY=6 | LINES=18 */

pub type Goal<'tcx, P> = ir::solve::Goal<TyCtxt<'tcx>, P>;
pub type QueryInput<'tcx, P> = ir::solve::QueryInput<TyCtxt<'tcx>, P>;
pub type QueryResult<'tcx> = ir::solve::QueryResult<TyCtxt<'tcx>>;
pub type CandidateSource<'tcx> = ir::solve::CandidateSource<TyCtxt<'tcx>>;
pub type CanonicalInput<'tcx, P = ty::Predicate<'tcx>> = ir::solve::CanonicalInput<TyCtxt<'tcx>, P>;
pub type CanonicalResponse<'tcx> = ir::solve::CanonicalResponse<TyCtxt<'tcx>>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, HashStable)]
pub struct PredefinedOpaques<'tcx>(pub(crate) Interned<'tcx, PredefinedOpaquesData<TyCtxt<'tcx>>>);

impl<'tcx> std::ops::Deref for PredefinedOpaques<'tcx> {
    type Target = PredefinedOpaquesData<TyCtxt<'tcx>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=ExternalConstraints | COMPLEXITY=5 | LINES=13 */

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, HashStable)]
pub struct ExternalConstraints<'tcx>(
    pub(crate) Interned<'tcx, ExternalConstraintsData<TyCtxt<'tcx>>>,
);

impl<'tcx> std::ops::Deref for ExternalConstraints<'tcx> {
    type Target = ExternalConstraintsData<TyCtxt<'tcx>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=20 | LINES=47 */

// FIXME: Having to clone `region_constraints` for folding feels bad and
// probably isn't great wrt performance.
//
// Not sure how to fix this, maybe we should also intern `opaque_types` and
// `region_constraints` here or something.
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for ExternalConstraints<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        // Perf testing has found that this check is slightly faster than
        // folding and re-interning an empty `ExternalConstraintsData`.
        // See: <https://github.com/rust-lang/rust/pull/142430>.
        if self.is_empty() {
            return Ok(self);
        }

        Ok(FallibleTypeFolder::cx(folder).mk_external_constraints(ExternalConstraintsData {
            region_constraints: self.region_constraints.clone().try_fold_with(folder)?,
            opaque_types: self
                .opaque_types
                .iter()
                .map(|opaque| opaque.try_fold_with(folder))
                .collect::<Result<_, F::Error>>()?,
            normalization_nested_goals: self
                .normalization_nested_goals
                .clone()
                .try_fold_with(folder)?,
        }))
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        // Perf testing has found that this check is slightly faster than
        // folding and re-interning an empty `ExternalConstraintsData`.
        // See: <https://github.com/rust-lang/rust/pull/142430>.
        if self.is_empty() {
            return self;
        }

        TypeFolder::cx(folder).mk_external_constraints(ExternalConstraintsData {
            region_constraints: self.region_constraints.clone().fold_with(folder),
            opaque_types: self.opaque_types.iter().map(|opaque| opaque.fold_with(folder)).collect(),
            normalization_nested_goals: self.normalization_nested_goals.clone().fold_with(folder),
        })
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=8 */

impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for ExternalConstraints<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        try_visit!(self.region_constraints.visit_with(visitor));
        try_visit!(self.opaque_types.visit_with(visitor));
        self.normalization_nested_goals.visit_with(visitor)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=12 | LINES=26 */

// FIXME: Having to clone `region_constraints` for folding feels bad and
// probably isn't great wrt performance.
//
// Not sure how to fix this, maybe we should also intern `opaque_types` and
// `region_constraints` here or something.
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for PredefinedOpaques<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        Ok(FallibleTypeFolder::cx(folder).mk_predefined_opaques_in_body(PredefinedOpaquesData {
            opaque_types: self
                .opaque_types
                .iter()
                .map(|opaque| opaque.try_fold_with(folder))
                .collect::<Result<_, F::Error>>()?,
        }))
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        TypeFolder::cx(folder).mk_predefined_opaques_in_body(PredefinedOpaquesData {
            opaque_types: self.opaque_types.iter().map(|opaque| opaque.fold_with(folder)).collect(),
        })
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6 */

impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for PredefinedOpaques<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        self.opaque_types.visit_with(visitor)
    }
}