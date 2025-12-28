macro_rules! deps {
    () => {
        NodeIndex!();
    };
}

macro_rules! StableGraphNode {
    () => {
        deps!();
        # [derive (Debug)] pub struct StableGraphNode < N , Ix > { pub index : NodeIndex < Ix > , pub weight : N , }
    };
}

StableGraphNode!();