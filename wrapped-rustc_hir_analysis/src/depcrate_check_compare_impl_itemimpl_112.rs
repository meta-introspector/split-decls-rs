// Generated macro for impl_112 (impl)
macro_rules! Depcrate_check_compare_impl_itemimpl_112 {
() => {
// Module: crate::check::compare_impl_item
// Provides: {"impl_112"}
// Dependencies: {}
impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for RemapLateParam < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { if let ty :: ReLateParam (fr) = r . kind () { ty :: Region :: new_late_param (self . tcx , fr . scope , self . mapping . get (& fr . kind) . copied () . unwrap_or (fr . kind) ,) } else { r } } }
};
}
