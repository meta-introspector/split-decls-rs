// Generated macro for impl_214 (impl)
macro_rules! Depcrate_hirimpl_214 {
() => {
// Module: crate::hir
// Provides: {"impl_214"}
// Dependencies: {}
impl < 'hir > ConstArg < 'hir , AmbigArg > { # [doc = " Converts a `ConstArg` in an ambiguous position to one in an unambiguous position."] # [doc = ""] # [doc = " Functions accepting unambiguous consts may expect the [`ConstArgKind::Infer`] variant"] # [doc = " to be used. Care should be taken to separately handle infer consts when calling this"] # [doc = " function as it cannot be handled by downstream code making use of the returned const."] # [doc = ""] # [doc = " In practice this may mean overriding the [`Visitor::visit_infer`][visit_infer] method on hir visitors, or"] # [doc = " specifically matching on [`GenericArg::Infer`] when handling generic arguments."] # [doc = ""] # [doc = " [visit_infer]: [rustc_hir::intravisit::Visitor::visit_infer]"] pub fn as_unambig_ct (& self) -> & ConstArg < 'hir > { let ptr = self as * const ConstArg < 'hir , AmbigArg > as * const ConstArg < 'hir , () > ; unsafe { & * ptr } } }
};
}
