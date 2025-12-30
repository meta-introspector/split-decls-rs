// Generated macro for FasNode (struct)
macro_rules! Depcrate_algo_feedback_arc_setFasNode {
() => {
// Module: crate::algo::feedback_arc_set
// Provides: {"FasNode"}
// Dependencies: {}
# [doc = " Represents a node from the input graph, tracking its current delta degree"] # [derive (Debug)] struct FasNode { # [doc = " Node index in input graph."] graph_ix : NodeIndex < usize > , # [doc = " All outward edges from this node (not removed during processing)"] out_edges : Vec < FasNodeIndex > , # [doc = " All inward edges from this node (not removed during processing)"] in_edges : Vec < FasNodeIndex > , # [doc = " Current out-degree of this node (decremented during processing as connected nodes are"] # [doc = " removed)"] out_degree : usize , # [doc = " Current in-degree of this node (decremented during processing as connected nodes are"] # [doc = " removed)"] in_degree : usize , }
};
}
