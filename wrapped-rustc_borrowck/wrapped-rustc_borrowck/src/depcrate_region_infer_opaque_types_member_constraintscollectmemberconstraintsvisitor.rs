// Generated macro for CollectMemberConstraintsVisitor (struct)
macro_rules! Depcrate_region_infer_opaque_types_member_constraintsCollectMemberConstraintsVisitor {
() => {
// Module: crate::region_infer::opaque_types::member_constraints
// Provides: {"CollectMemberConstraintsVisitor"}
// Dependencies: {}
struct CollectMemberConstraintsVisitor < 'a , 'b , 'tcx > { rcx : & 'a RegionCtxt < 'a , 'tcx > , defining_use : & 'b DefiningUse < 'tcx > , member_constraints : & 'a mut FxHashMap < ConstraintSccIndex , Vec < & 'b DefiningUse < 'tcx > > > , }
};
}
