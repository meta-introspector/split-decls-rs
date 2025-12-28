macro_rules! deps {
    () => {
        NodeIndex!();
        EdgeIndex!();
        Create!();
        StableGraph!();
        Graph!();
        IndexType!();
    };
}

macro_rules! impl_808 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > StableGraph < N , E , Ty , Ix > where Ix : IndexType , { # [doc = " Create a new `StableGraph` with estimated capacity."] pub fn with_capacity (nodes : usize , edges : usize) -> Self { StableGraph { g : Graph :: with_capacity (nodes , edges) , node_count : 0 , edge_count : 0 , free_node : NodeIndex :: end () , free_edge : EdgeIndex :: end () , } } }
    };
}

impl_808!()