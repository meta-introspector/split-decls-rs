macro_rules! deps {
    () => {
        NodeFiltered!();
        NodeIdentifiers!();
        NodeFilteredNeighbors!();
        FilterNode!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'a , G , F > IntoNodeIdentifiers for & 'a NodeFiltered < G , F > where G : IntoNodeIdentifiers , F : FilterNode < G :: NodeId > , { type NodeIdentifiers = NodeFilteredNeighbors < 'a , G :: NodeIdentifiers , F > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { NodeFilteredNeighbors { include_source : true , iter : self . 0 . node_identifiers () , f : & self . 1 , } } }
    };
}

impl_124!()