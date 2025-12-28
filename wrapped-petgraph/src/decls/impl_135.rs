macro_rules! deps {
    () => {
        NodeFiltered!();
        FilterNode!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < G , F > DataMap for NodeFiltered < G , F > where G : DataMap , F : FilterNode < G :: NodeId > , { fn node_weight (& self , id : Self :: NodeId) -> Option < & Self :: NodeWeight > { if self . 1 . include_node (id) { self . 0 . node_weight (id) } else { None } } fn edge_weight (& self , id : Self :: EdgeId) -> Option < & Self :: EdgeWeight > { self . 0 . edge_weight (id) } }
    };
}

impl_135!()