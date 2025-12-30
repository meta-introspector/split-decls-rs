// Generated macro for impl_248 (impl)
macro_rules! Depcrate_check_wfcheckimpl_248 {
() => {
// Module: crate::check::wfcheck
// Provides: {"impl_248"}
// Dependencies: {}
impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for GATArgsCollector < 'tcx > { fn visit_ty (& mut self , t : Ty < 'tcx >) { match t . kind () { ty :: Alias (ty :: Projection , p) if p . def_id == self . gat => { for (idx , arg) in p . args . iter () . enumerate () { match arg . kind () { GenericArgKind :: Lifetime (lt) if ! lt . is_bound () => { self . regions . insert ((lt , idx)) ; } GenericArgKind :: Type (t) => { self . types . insert ((t , idx)) ; } _ => { } } } } _ => { } } t . super_visit_with (self) } }
};
}
