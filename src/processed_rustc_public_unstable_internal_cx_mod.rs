// SRC: ../rust/compiler/rustc_public/src/unstable/internal_cx/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
// Implementation of InternalCx.

pub(crate) use helpers::*;
use crate::rustc_complete::ty::{List, Ty, TyCtxt};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{mir, ty};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=new_from_args | COMPLEXITY=5 | LINES=15 */

use super::InternalCx;


impl<'tcx, T: InternalCx<'tcx>> ExistentialProjectionHelpers<'tcx> for T {
    fn new_from_args(
        &self,
        def_id: crate::rustc_span::def_id::DefId,
        args: ty::GenericArgsRef<'tcx>,
        term: ty::Term<'tcx>,
    ) -> ty::ExistentialProjection<'tcx> {
        ty::ExistentialProjection::new_from_args(self.tcx(), def_id, args, term)
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=new_from_args | COMPLEXITY=5 | LINES=10 */

impl<'tcx, T: InternalCx<'tcx>> ExistentialTraitRefHelpers<'tcx> for T {
    fn new_from_args(
        &self,
        trait_def_id: crate::rustc_span::def_id::DefId,
        args: ty::GenericArgsRef<'tcx>,
    ) -> ty::ExistentialTraitRef<'tcx> {
        ty::ExistentialTraitRef::new_from_args(self.tcx(), trait_def_id, args)
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=new_from_args | COMPLEXITY=5 | LINES=10 */

impl<'tcx, T: InternalCx<'tcx>> TraitRefHelpers<'tcx> for T {
    fn new_from_args(
        &self,
        trait_def_id: crate::rustc_span::def_id::DefId,
        args: ty::GenericArgsRef<'tcx>,
    ) -> ty::TraitRef<'tcx> {
        ty::TraitRef::new_from_args(self.tcx(), trait_def_id, args)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=tcx | COMPLEXITY=17 | LINES=53 */

impl<'tcx> InternalCx<'tcx> for TyCtxt<'tcx> {
    fn tcx(self) -> TyCtxt<'tcx> {
        self
    }

    fn lift<T: ty::Lift<TyCtxt<'tcx>>>(self, value: T) -> Option<T::Lifted> {
        TyCtxt::lift(self, value)
    }

    fn mk_args_from_iter<I, T>(self, iter: I) -> T::Output
    where
        I: Iterator<Item = T>,
        T: ty::CollectAndApply<ty::GenericArg<'tcx>, ty::GenericArgsRef<'tcx>>,
    {
        TyCtxt::mk_args_from_iter(self, iter)
    }

    fn mk_pat(self, v: ty::PatternKind<'tcx>) -> ty::Pattern<'tcx> {
        TyCtxt::mk_pat(self, v)
    }

    fn mk_poly_existential_predicates(
        self,
        eps: &[ty::PolyExistentialPredicate<'tcx>],
    ) -> &'tcx List<ty::PolyExistentialPredicate<'tcx>> {
        TyCtxt::mk_poly_existential_predicates(self, eps)
    }

    fn mk_type_list(self, v: &[Ty<'tcx>]) -> &'tcx List<Ty<'tcx>> {
        TyCtxt::mk_type_list(self, v)
    }

    fn lifetimes_re_erased(self) -> ty::Region<'tcx> {
        self.lifetimes.re_erased
    }

    fn mk_bound_variable_kinds_from_iter<I, T>(self, iter: I) -> T::Output
    where
        I: Iterator<Item = T>,
        T: ty::CollectAndApply<ty::BoundVariableKind, &'tcx List<ty::BoundVariableKind>>,
    {
        TyCtxt::mk_bound_variable_kinds_from_iter(self, iter)
    }

    fn mk_place_elems(self, v: &[mir::PlaceElem<'tcx>]) -> &'tcx List<mir::PlaceElem<'tcx>> {
        TyCtxt::mk_place_elems(self, v)
    }

    fn adt_def(self, def_id: crate::rustc_hir::def_id::DefId) -> ty::AdtDef<'tcx> {
        self.adt_def(def_id)
    }
}