// SRC: ../rust/compiler/rustc_public/src/unstable/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */
// Module that collects the things that have no stability guarantees.
//
// We want to keep rustc_public's IR definitions and logic separate from
// any sort of conversion and usage of internal rustc code. So we
// restrict the usage of internal items to be inside this module.

use std::marker::PointeeSized;

use crate::rustc_complete::def::DefKind;
use crate::rustc_complete::ty::{List, Ty, TyCtxt};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::{mir, ty};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
use crate::rustc_public_bridge::Tables;
use crate::rustc_public_bridge::context::CompilerCtxt;

use super::compiler_interface::BridgeTys;
use crate::{CtorKind, ItemKind};
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=tcx | COMPLEXITY=6 | LINES=38 */


/// Trait that defines the methods that are fine to call from [`RustcInternal`].
///
/// This trait is only for [`RustcInternal`]. Any other other access to rustc's internals
/// should go through [`crate::rustc_public_bridge::context::CompilerCtxt`].
pub trait InternalCx<'tcx>: Copy + Clone {
    fn tcx(self) -> TyCtxt<'tcx>;

    fn lift<T: ty::Lift<TyCtxt<'tcx>>>(self, value: T) -> Option<T::Lifted>;

    fn mk_args_from_iter<I, T>(self, iter: I) -> T::Output
    where
        I: Iterator<Item = T>,
        T: ty::CollectAndApply<ty::GenericArg<'tcx>, ty::GenericArgsRef<'tcx>>;

    fn mk_pat(self, v: ty::PatternKind<'tcx>) -> ty::Pattern<'tcx>;

    fn mk_poly_existential_predicates(
        self,
        eps: &[ty::PolyExistentialPredicate<'tcx>],
    ) -> &'tcx List<ty::PolyExistentialPredicate<'tcx>>;

    fn mk_type_list(self, v: &[Ty<'tcx>]) -> &'tcx List<Ty<'tcx>>;

    fn lifetimes_re_erased(self) -> ty::Region<'tcx>;

    fn mk_bound_variable_kinds_from_iter<I, T>(self, iter: I) -> T::Output
    where
        I: Iterator<Item = T>,
        T: ty::CollectAndApply<ty::BoundVariableKind, &'tcx List<ty::BoundVariableKind>>;

    fn mk_place_elems(self, v: &[mir::PlaceElem<'tcx>]) -> &'tcx List<mir::PlaceElem<'tcx>>;

    fn adt_def(self, def_id: crate::rustc_hir::def_id::DefId) -> ty::AdtDef<'tcx>;
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=stable | COMPLEXITY=3 | LINES=17 */

/// Trait used to convert between an internal MIR type to a rustc_public's IR type.
///
/// This trait is currently exposed to users so they can have interoperability
/// between internal MIR and rustc_public's IR constructs.
/// However, they should be used seldom and they have no influence in this crate semver.
#[doc(hidden)]
pub trait Stable<'tcx>: PointeeSized {
    /// The stable representation of the type implementing Stable.
    type T;
    /// Converts an object to the equivalent rustc_public's IR representation.
    fn stable<'cx>(
        &self,
        tables: &mut Tables<'cx, BridgeTys>,
        cx: &CompilerCtxt<'cx, BridgeTys>,
    ) -> Self::T;
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=internal | COMPLEXITY=3 | LINES=17 */

/// Trait used to translate a rustc_public's IR construct to its rustc counterpart.
///
/// This is basically a mirror of [Stable].
///
/// This trait is currently exposed to users so they can have interoperability
/// between internal MIR and rustc_public's IR constructs.
/// They should be used seldom as they have no stability guarantees.
#[doc(hidden)]
pub trait RustcInternal {
    type T<'tcx>;
    fn internal<'tcx>(
        &self,
        tables: &mut Tables<'_, BridgeTys>,
        tcx: impl InternalCx<'tcx>,
    ) -> Self::T<'tcx>;
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=37 */

pub(crate) fn new_item_kind(kind: DefKind) -> ItemKind {
    match kind {
        DefKind::Mod
        | DefKind::Struct
        | DefKind::Union
        | DefKind::Enum
        | DefKind::Variant
        | DefKind::Trait
        | DefKind::TyAlias
        | DefKind::ForeignTy
        | DefKind::TraitAlias
        | DefKind::AssocTy
        | DefKind::TyParam
        | DefKind::ConstParam
        | DefKind::Macro(_)
        | DefKind::ExternCrate
        | DefKind::Use
        | DefKind::ForeignMod
        | DefKind::OpaqueTy
        | DefKind::Field
        | DefKind::LifetimeParam
        | DefKind::Impl { .. }
        | DefKind::GlobalAsm => {
            unreachable!("Not a valid item kind: {kind:?}");
        }
        DefKind::Closure | DefKind::AssocFn | DefKind::Fn | DefKind::SyntheticCoroutineBody => {
            ItemKind::Fn
        }
        DefKind::Const | DefKind::InlineConst | DefKind::AssocConst | DefKind::AnonConst => {
            ItemKind::Const
        }
        DefKind::Static { .. } => ItemKind::Static,
        DefKind::Ctor(_, crate::rustc_hir::def::CtorKind::Const) => ItemKind::Ctor(CtorKind::Const),
        DefKind::Ctor(_, crate::rustc_hir::def::CtorKind::Fn) => ItemKind::Ctor(CtorKind::Fn),
    }
}