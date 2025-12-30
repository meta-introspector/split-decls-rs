// Generated macro for impl_905 (impl)
macro_rules! Depcrate_region_infer_opaque_types_member_constraintsimpl_905 {
() => {
// Module: crate::region_infer::opaque_types::member_constraints
// Provides: {"impl_905"}
// Dependencies: {}
impl < 'tcx > CollectMemberConstraintsVisitor < '_ , '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . rcx . infcx . tcx } fn visit_closure_args (& mut self , def_id : DefId , args : GenericArgsRef < 'tcx >) { let generics = self . cx () . generics_of (def_id) ; for arg in args . iter () . skip (generics . parent_count) { arg . visit_with (self) ; } } }
};
}
