// SRC: ../rust/compiler/rustc_public/src/unstable/internal_cx/helpers.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=new_from_args | COMPLEXITY=2 | LINES=15 */
// A set of traits that define a stable interface to rustc's internals.
//
// These traits are primarily used to clarify the behavior of different
// functions that share the same name across various contexts.

use crate::rustc_complete::ty;

pub(crate) trait ExistentialProjectionHelpers<'tcx> {
    fn new_from_args(
        &self,
        def_id: crate::rustc_span::def_id::DefId,
        args: ty::GenericArgsRef<'tcx>,
        term: ty::Term<'tcx>,
    ) -> ty::ExistentialProjection<'tcx>;
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=new_from_args | COMPLEXITY=2 | LINES=8 */

pub(crate) trait ExistentialTraitRefHelpers<'tcx> {
    fn new_from_args(
        &self,
        trait_def_id: crate::rustc_span::def_id::DefId,
        args: ty::GenericArgsRef<'tcx>,
    ) -> ty::ExistentialTraitRef<'tcx>;
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=new_from_args | COMPLEXITY=2 | LINES=8 */

pub(crate) trait TraitRefHelpers<'tcx> {
    fn new_from_args(
        &self,
        trait_def_id: crate::rustc_span::def_id::DefId,
        args: ty::GenericArgsRef<'tcx>,
    ) -> ty::TraitRef<'tcx>;
}