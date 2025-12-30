// Generated macro for impl_906 (impl)
macro_rules! Depcrate_region_infer_opaque_types_member_constraintsimpl_906 {
() => {
// Module: crate::region_infer::opaque_types::member_constraints
// Provides: {"impl_906"}
// Dependencies: {}
impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for CollectMemberConstraintsVisitor < '_ , '_ , 'tcx > { fn visit_region (& mut self , r : Region < 'tcx >) { match r . kind () { ty :: ReBound (..) => return , ty :: ReVar (vid) => { let scc = self . rcx . constraint_sccs . scc (vid) ; self . member_constraints . entry (scc) . or_default () . push (self . defining_use) ; } _ => unreachable ! () , } } fn visit_ty (& mut self , ty : Ty < 'tcx >) { if ! ty . flags () . intersects (ty :: TypeFlags :: HAS_FREE_REGIONS) { return ; } match * ty . kind () { ty :: Closure (def_id , args) | ty :: CoroutineClosure (def_id , args) | ty :: Coroutine (def_id , args) => self . visit_closure_args (def_id , args) , ty :: Alias (kind , ty :: AliasTy { def_id , args , .. }) if let Some (variances) = self . cx () . opt_alias_variances (kind , def_id) => { for (& v , arg) in std :: iter :: zip (variances , args . iter ()) { if v != ty :: Bivariant { arg . visit_with (self) } } } _ => ty . super_visit_with (self) , } } }
};
}
