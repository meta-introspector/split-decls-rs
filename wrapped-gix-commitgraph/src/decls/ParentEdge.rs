macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! ParentEdge {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] enum ParentEdge { None , GraphPosition (Position) , ExtraEdgeIndex (u32) , }
    };
}

ParentEdge!()