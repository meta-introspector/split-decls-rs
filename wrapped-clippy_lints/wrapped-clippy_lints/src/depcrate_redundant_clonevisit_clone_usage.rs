// Generated macro for visit_clone_usage (function)
macro_rules! Depcrate_redundant_clonevisit_clone_usage {
() => {
// Module: crate::redundant_clone
// Provides: {"visit_clone_usage"}
// Dependencies: {}
fn visit_clone_usage (cloned : mir :: Local , clone : mir :: Local , mir : & mir :: Body < '_ > , bb : mir :: BasicBlock) -> CloneUsage { if let Some ((LocalUsage { local_use_locs : cloned_use_locs , local_consume_or_mutate_locs : cloned_consume_or_mutate_locs , } , LocalUsage { local_use_locs : _ , local_consume_or_mutate_locs : clone_consume_or_mutate_locs , } ,)) = visit_local_usage (& [cloned , clone] , mir , mir :: Location { block : bb , statement_index : mir . basic_blocks [bb] . statements . len () , } ,) . map (| mut vec | (vec . remove (0) , vec . remove (0))) { CloneUsage { cloned_use_loc : cloned_use_locs . first () . copied () . into () , cloned_consume_or_mutate_loc : cloned_consume_or_mutate_locs . first () . copied () , clone_consumed_or_mutated : mir . local_kind (clone) != mir :: LocalKind :: Temp || ! clone_consume_or_mutate_locs . is_empty () , } } else { CloneUsage { cloned_use_loc : MirLocalUsage :: Unknown , cloned_consume_or_mutate_loc : None , clone_consumed_or_mutated : true , } } }
};
}
