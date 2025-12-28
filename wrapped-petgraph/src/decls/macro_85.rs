macro_rules! deps {
    () => {
        NodeRef!();
        NodeReferences!();
    };
}

macro_rules! macro_85 {
    () => {
        deps!();
        trait_template ! { # [doc = " Access to the sequence of the graph’s nodes"] pub trait IntoNodeReferences : Data + IntoNodeIdentifiers { @ section type type NodeRef : NodeRef < NodeId = Self :: NodeId , Weight = Self :: NodeWeight >; type NodeReferences : Iterator < Item = Self :: NodeRef >; @ section self fn node_references (self) -> Self :: NodeReferences ; } }
    };
}

macro_85!()