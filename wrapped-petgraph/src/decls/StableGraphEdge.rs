macro_rules! deps {
    () => {
        EdgeIndex!();
        NodeIndex!();
    };
}

macro_rules! StableGraphEdge {
    () => {
        deps!();
        # [derive (Debug)] pub struct StableGraphEdge < E , Ix > { pub index : EdgeIndex < Ix > , pub source : NodeIndex < Ix > , pub target : NodeIndex < Ix > , pub weight : E , }
    };
}

StableGraphEdge!();