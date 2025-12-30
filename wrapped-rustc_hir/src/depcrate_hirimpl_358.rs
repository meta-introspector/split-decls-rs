// Generated macro for impl_358 (impl)
macro_rules! Depcrate_hirimpl_358 {
() => {
// Module: crate::hir
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'hir > Ty < 'hir , AmbigArg > { # [doc = " Converts a `Ty` in an ambiguous position to one in an unambiguous position."] # [doc = ""] # [doc = " Functions accepting an unambiguous types may expect the [`TyKind::Infer`] variant"] # [doc = " to be used. Care should be taken to separately handle infer types when calling this"] # [doc = " function as it cannot be handled by downstream code making use of the returned ty."] # [doc = ""] # [doc = " In practice this may mean overriding the [`Visitor::visit_infer`][visit_infer] method on hir visitors, or"] # [doc = " specifically matching on [`GenericArg::Infer`] when handling generic arguments."] # [doc = ""] # [doc = " [visit_infer]: [rustc_hir::intravisit::Visitor::visit_infer]"] pub fn as_unambig_ty (& self) -> & Ty < 'hir > { let ptr = self as * const Ty < 'hir , AmbigArg > as * const Ty < 'hir , () > ; unsafe { & * ptr } } }
};
}
