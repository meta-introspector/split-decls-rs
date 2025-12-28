macro_rules! deps {
    () => {
        NodeIndex!();
        FasNodeIndex!();
        Node!();
    };
}

macro_rules! FasNode {
    () => {
        deps!();
        # [doc = " Represents a node from the input graph, tracking its current delta degree"] # [derive (Debug)] struct FasNode { # [doc = " Node index in input graph."] graph_ix : NodeIndex < usize > , # [doc = " All outward edges from this node (not removed during processing)"] out_edges : Vec < FasNodeIndex > , # [doc = " All inward edges from this node (not removed during processing)"] in_edges : Vec < FasNodeIndex > , # [doc = " Current out-degree of this node (decremented during processing as connected nodes are"] # [doc = " removed)"] out_degree : usize , # [doc = " Current in-degree of this node (decremented during processing as connected nodes are"] # [doc = " removed)"] in_degree : usize , }
    };
}

FasNode!();