macro_rules! deps {
    () => {
        DefaultIx!();
        Node!();
    };
}

macro_rules! NodeIndex {
    () => {
        deps!();
        # [doc = " Node identifier."] pub type NodeIndex < Ix = DefaultIx > = GraphNodeIndex < Ix > ;
    };
}

NodeIndex!();