// Generated macro for impl_273 (impl)
macro_rules! Depcrate_const_eval_machineimpl_273 {
() => {
// Module: crate::const_eval::machine
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'tcx > CompileTimeMachine < 'tcx > { pub (crate) fn new (can_access_mut_global : CanAccessMutGlobal , check_alignment : CheckAlignment ,) -> Self { CompileTimeMachine { num_evaluated_steps : 0 , stack : Vec :: new () , can_access_mut_global , check_alignment , static_root_ids : None , union_data_ranges : FxHashMap :: default () , } } }
};
}
