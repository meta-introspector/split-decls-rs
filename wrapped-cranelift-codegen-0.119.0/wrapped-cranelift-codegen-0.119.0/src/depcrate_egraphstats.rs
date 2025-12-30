// Generated macro for Stats (struct)
macro_rules! Depcrate_egraphStats {
() => {
// Module: crate::egraph
// Provides: {"Stats"}
// Dependencies: {}
# [doc = " Statistics collected during egraph-based processing."] # [derive (Clone , Debug , Default)] pub (crate) struct Stats { pub (crate) pure_inst : u64 , pub (crate) pure_inst_deduped : u64 , pub (crate) pure_inst_subsume : u64 , pub (crate) pure_inst_rewrite_to_self : u64 , pub (crate) pure_inst_insert_orig : u64 , pub (crate) pure_inst_insert_new : u64 , pub (crate) skeleton_inst : u64 , pub (crate) skeleton_inst_gvn : u64 , pub (crate) alias_analysis_removed : u64 , pub (crate) new_inst : u64 , pub (crate) union : u64 , pub (crate) subsume : u64 , pub (crate) remat : u64 , pub (crate) rewrite_rule_invoked : u64 , pub (crate) rewrite_rule_results : u64 , pub (crate) rewrite_depth_limit : u64 , pub (crate) elaborate_visit_node : u64 , pub (crate) elaborate_memoize_hit : u64 , pub (crate) elaborate_memoize_miss : u64 , pub (crate) elaborate_remat : u64 , pub (crate) elaborate_licm_hoist : u64 , pub (crate) elaborate_func : u64 , pub (crate) elaborate_func_pre_insts : u64 , pub (crate) elaborate_func_post_insts : u64 , pub (crate) elaborate_best_cost_fixpoint_iters : u64 , }
};
}
