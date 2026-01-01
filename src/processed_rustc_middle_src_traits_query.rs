// SRC: ../rust/compiler/rustc_middle/src/traits/query.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=6 | LINES=8 */
// Experimental types for the trait query interface. The methods
// defined in this module are all based on **canonicalization**,
// which makes a canonical query by replacing unbound inference
// variables and regions, so that results can be reused more broadly.
// The providers for the queries defined here can be found in
// `rustc_traits`.

use rustc_macros::{HashStable, TypeFoldable, TypeVisitable};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use crate::rustc_complete::Span;

use crate::error::DropCheckOverflow;
use crate::infer::canonical::{Canonical, CanonicalQueryInput, QueryResponse};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
pub use crate::traits::solve::NoSolution;
use crate::ty::{self, GenericArg, Ty, TyCtxt};
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=AscribeUserType | COMPLEXITY=15 | LINES=51 */

pub mod type_op {
    use rustc_macros::{HashStable, TypeFoldable, TypeVisitable};

    use crate::ty::{Predicate, Ty, UserType};

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, HashStable, TypeFoldable, TypeVisitable)]
    pub struct AscribeUserType<'tcx> {
        pub mir_ty: Ty<'tcx>,
        pub user_ty: UserType<'tcx>,
    }

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, HashStable, TypeFoldable, TypeVisitable)]
    pub struct Eq<'tcx> {
        pub a: Ty<'tcx>,
        pub b: Ty<'tcx>,
    }

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, HashStable, TypeFoldable, TypeVisitable)]
    pub struct Subtype<'tcx> {
        pub sub: Ty<'tcx>,
        pub sup: Ty<'tcx>,
    }

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, HashStable, TypeFoldable, TypeVisitable)]
    pub struct ProvePredicate<'tcx> {
        pub predicate: Predicate<'tcx>,
    }

    /// Normalizes, but not in the new solver.
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, HashStable, TypeFoldable, TypeVisitable)]
    pub struct Normalize<T> {
        pub value: T,
    }

    /// Normalizes, and deeply normalizes in the new solver.
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, HashStable, TypeFoldable, TypeVisitable)]
    pub struct DeeplyNormalize<T> {
        pub value: T,
    }

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, HashStable, TypeFoldable, TypeVisitable)]
    pub struct ImpliedOutlivesBounds<'tcx> {
        pub ty: Ty<'tcx>,
    }

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, HashStable, TypeFoldable, TypeVisitable)]
    pub struct DropckOutlives<'tcx> {
        pub dropped_ty: Ty<'tcx>,
    }
}
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=DropckOutlivesResult | COMPLEXITY=4 | LINES=32 */

pub type CanonicalAliasGoal<'tcx> =
    CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, ty::AliasTy<'tcx>>>;

pub type CanonicalTyGoal<'tcx> = CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, Ty<'tcx>>>;

pub type CanonicalPredicateGoal<'tcx> =
    CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, ty::Predicate<'tcx>>>;

pub type CanonicalTypeOpAscribeUserTypeGoal<'tcx> =
    CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, type_op::AscribeUserType<'tcx>>>;

pub type CanonicalTypeOpProvePredicateGoal<'tcx> =
    CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, type_op::ProvePredicate<'tcx>>>;

pub type CanonicalTypeOpNormalizeGoal<'tcx, T> =
    CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, type_op::Normalize<T>>>;

pub type CanonicalTypeOpDeeplyNormalizeGoal<'tcx, T> =
    CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, type_op::DeeplyNormalize<T>>>;

pub type CanonicalImpliedOutlivesBoundsGoal<'tcx> =
    CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, type_op::ImpliedOutlivesBounds<'tcx>>>;

pub type CanonicalDropckOutlivesGoal<'tcx> =
    CanonicalQueryInput<'tcx, ty::ParamEnvAnd<'tcx, type_op::DropckOutlives<'tcx>>>;

#[derive(Clone, Debug, Default, HashStable, TypeFoldable, TypeVisitable)]
pub struct DropckOutlivesResult<'tcx> {
    pub kinds: Vec<GenericArg<'tcx>>,
    pub overflows: Vec<Ty<'tcx>>,
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=report_overflows | COMPLEXITY=7 | LINES=8 */

impl<'tcx> DropckOutlivesResult<'tcx> {
    pub fn report_overflows(&self, tcx: TyCtxt<'tcx>, span: Span, ty: Ty<'tcx>) {
        if let Some(overflow_ty) = self.overflows.get(0) {
            tcx.dcx().emit_err(DropCheckOverflow { span, ty, overflow_ty: *overflow_ty });
        }
    }
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=DropckConstraint | COMPLEXITY=9 | LINES=17 */

/// A set of constraints that need to be satisfied in order for
/// a type to be valid for destruction.
#[derive(Clone, Debug, HashStable)]
pub struct DropckConstraint<'tcx> {
    /// Types that are required to be alive in order for this
    /// type to be valid for destruction.
    pub outlives: Vec<ty::GenericArg<'tcx>>,

    /// Types that could not be resolved: projections and params.
    pub dtorck_types: Vec<Ty<'tcx>>,

    /// If, during the computation of the dtorck constraint, we
    /// overflow, that gets recorded here. The caller is expected to
    /// report an error.
    pub overflows: Vec<Ty<'tcx>>,
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=empty | COMPLEXITY=4 | LINES=6 */

impl<'tcx> DropckConstraint<'tcx> {
    pub fn empty() -> DropckConstraint<'tcx> {
        DropckConstraint { outlives: vec![], dtorck_types: vec![], overflows: vec![] }
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=9 | LINES=14 */

impl<'tcx> FromIterator<DropckConstraint<'tcx>> for DropckConstraint<'tcx> {
    fn from_iter<I: IntoIterator<Item = DropckConstraint<'tcx>>>(iter: I) -> Self {
        let mut result = Self::empty();

        for DropckConstraint { outlives, dtorck_types, overflows } in iter {
            result.outlives.extend(outlives);
            result.dtorck_types.extend(dtorck_types);
            result.overflows.extend(overflows);
        }

        result
    }
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=CandidateStep | COMPLEXITY=10 | LINES=22 */

#[derive(Debug, HashStable)]
pub struct CandidateStep<'tcx> {
    pub self_ty: Canonical<'tcx, QueryResponse<'tcx, Ty<'tcx>>>,
    pub autoderefs: usize,
    /// `true` if the type results from a dereference of a raw pointer.
    /// when assembling candidates, we include these steps, but not when
    /// picking methods. This so that if we have `foo: *const Foo` and `Foo` has methods
    /// `fn by_raw_ptr(self: *const Self)` and `fn by_ref(&self)`, then
    /// `foo.by_raw_ptr()` will work and `foo.by_ref()` won't.
    pub from_unsafe_deref: bool,
    pub unsize: bool,
    /// We will generate CandidateSteps which are reachable via a chain
    /// of following `Receiver`. The first 'n' of those will be reachable
    /// by following a chain of 'Deref' instead (since there's a blanket
    /// implementation of Receiver for Deref).
    /// We use the entire set of steps when identifying method candidates
    /// (e.g. identifying relevant `impl` blocks) but only those that are
    /// reachable via Deref when examining what the receiver type can
    /// be converted into by autodereffing.
    pub reachable_via_deref: bool,
}
/* AST_META: AST_ID=11 | TYPE=STRUCT | NAME=MethodAutoderefStepsResult | COMPLEXITY=3 | LINES=12 */

#[derive(Copy, Clone, Debug, HashStable)]
pub struct MethodAutoderefStepsResult<'tcx> {
    /// The valid autoderef steps that could be found by following a chain
    /// of `Receiver<Target=T>` or `Deref<Target=T>` trait implementations.
    pub steps: &'tcx [CandidateStep<'tcx>],
    /// If Some(T), a type autoderef reported an error on.
    pub opt_bad_ty: Option<&'tcx MethodAutoderefBadTy<'tcx>>,
    /// If `true`, `steps` has been truncated due to reaching the
    /// recursion limit.
    pub reached_recursion_limit: bool,
}
/* AST_META: AST_ID=12 | TYPE=STRUCT | NAME=MethodAutoderefBadTy | COMPLEXITY=2 | LINES=6 */

#[derive(Debug, HashStable)]
pub struct MethodAutoderefBadTy<'tcx> {
    pub reached_raw_pointer: bool,
    pub ty: Canonical<'tcx, QueryResponse<'tcx, Ty<'tcx>>>,
}
/* AST_META: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=2 */

/// Result of the `normalize_canonicalized_{{,inherent_}projection,free}_ty` queries.
/* AST_META: AST_ID=14 | TYPE=STRUCT | NAME=NormalizationResult | COMPLEXITY=2 | LINES=5 */
#[derive(Clone, Debug, HashStable, TypeFoldable, TypeVisitable)]
pub struct NormalizationResult<'tcx> {
    /// Result of the normalization.
    pub normalized_ty: Ty<'tcx>,
}
/* AST_META: AST_ID=15 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=14 */

/// Outlives bounds are relationships between generic parameters,
/// whether they both be regions (`'a: 'b`) or whether types are
/// involved (`T: 'a`). These relationships can be extracted from the
/// full set of predicates we understand or also from types (in which
/// case they are called implied bounds). They are fed to the
/// `OutlivesEnv` which in turn is supplied to the region checker and
/// other parts of the inference system.
#[derive(Copy, Clone, Debug, TypeFoldable, TypeVisitable, HashStable)]
pub enum OutlivesBound<'tcx> {
    RegionSubRegion(ty::Region<'tcx>, ty::Region<'tcx>),
    RegionSubParam(ty::Region<'tcx>, ty::ParamTy),
    RegionSubAlias(ty::Region<'tcx>, ty::AliasTy<'tcx>),
}