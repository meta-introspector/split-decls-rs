macro_rules! deps {
    () => {
        Node!();
        DefaultIx!();
    };
}

macro_rules! NodeIndex {
    () => {
        deps!();
        # [doc = " Node identifier."] pub type NodeIndex < Ix = DefaultIx > = GraphNodeIndex < Ix > ;
    };
}

NodeIndex!()