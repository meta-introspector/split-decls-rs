// Generated macro for post_order_dfs (function)
macro_rules! Depcrate_souper_harvestpost_order_dfs {
() => {
// Module: crate::souper_harvest
// Provides: {"post_order_dfs"}
// Dependencies: {}
fn post_order_dfs (allocs : & mut Allocs , dfg : & ir :: DataFlowGraph , val : ir :: Value , should_trace : impl Fn (ir :: Value) -> bool , mut visit : impl FnMut (& mut Allocs , ir :: Value) ,) { allocs . dfs_stack . push (StackEntry :: Trace (val)) ; while let Some (entry) = allocs . dfs_stack . pop () { match entry { StackEntry :: Visit (val) => { let is_new = allocs . dfs_seen . insert (val) ; if is_new { visit (allocs , val) ; } } StackEntry :: Trace (val) => { if allocs . dfs_seen . contains (& val) { continue ; } allocs . dfs_stack . push (StackEntry :: Visit (val)) ; if should_trace (val) { if let ir :: ValueDef :: Result (inst , 0) = dfg . value_def (val) { let args = dfg . inst_args (inst) ; for v in args . iter () . rev () . copied () { allocs . dfs_stack . push (StackEntry :: Trace (v)) ; } } } } } } }
};
}
