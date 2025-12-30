// Generated macro for WalkReturn (enum)
macro_rules! Depcrate_graph_sccWalkReturn {
() => {
// Module: crate::graph::scc
// Provides: {"WalkReturn"}
// Dependencies: {}
# [doc = " The state of walking a given node."] # [derive (Copy , Clone , Debug)] enum WalkReturn < S , A : Annotation > { # [doc = " The walk found a cycle, but the entire component is not known to have"] # [doc = " been fully walked yet. We only know the minimum depth of  this"] # [doc = " component in a minimum spanning tree of the graph. This component"] # [doc = " is tentatively represented by the state of the first node of this"] # [doc = " cycle we met, which is at `min_depth`."] Cycle { min_depth : usize , annotation : A } , # [doc = " The SCC and everything reachable from it have been fully walked."] # [doc = " At this point we know what is inside the SCC as we have visited every"] # [doc = " node reachable from it. The SCC can now be fully represented by its ID."] Complete { scc_index : S , annotation : A } , }
};
}
