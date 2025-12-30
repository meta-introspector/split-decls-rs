// Generated macro for OptimizeCtx (struct)
macro_rules! Depcrate_egraphOptimizeCtx {
() => {
// Module: crate::egraph
// Provides: {"OptimizeCtx"}
// Dependencies: {}
# [doc = " Context passed through node insertion and optimization."] pub (crate) struct OptimizeCtx < 'opt , 'analysis > where 'analysis : 'opt , { pub (crate) func : & 'opt mut Function , pub (crate) value_to_opt_value : & 'opt mut SecondaryMap < Value , Value > , available_block : & 'opt mut SecondaryMap < Value , Block > , pub (crate) gvn_map : & 'opt mut ScopedHashMap < (Type , InstructionData) , Option < Value > > , pub (crate) gvn_map_blocks : & 'opt Vec < Block > , pub (crate) remat_values : & 'opt mut FxHashSet < Value > , pub (crate) stats : & 'opt mut Stats , domtree : & 'opt DominatorTreePreorder , pub (crate) alias_analysis : & 'opt mut AliasAnalysis < 'analysis > , pub (crate) alias_analysis_state : & 'opt mut LastStores , flags : & 'opt Flags , ctrl_plane : & 'opt mut ControlPlane , pub (crate) rewrite_depth : usize , pub (crate) subsume_values : FxHashSet < Value > , optimized_values : SmallVec < [Value ; MATCHES_LIMIT] > , }
};
}
