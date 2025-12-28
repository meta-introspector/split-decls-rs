macro_rules! deps {
    () => {
        GraphRef!();
        NodeIdentifiers!();
    };
}

macro_rules! macro_76 {
    () => {
        deps!();
        trait_template ! { # [doc = " Access to the sequence of the graph’s `NodeId`s."] pub trait IntoNodeIdentifiers : GraphRef { @ section type type NodeIdentifiers : Iterator < Item = Self :: NodeId >; @ section self fn node_identifiers (self) -> Self :: NodeIdentifiers ; } }
    };
}

macro_76!();