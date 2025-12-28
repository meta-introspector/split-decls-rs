macro_rules! deps {
    () => {
        StableGraph!();
        EdgeIndex!();
        NodeIndex!();
        DefaultIx!();
    };
}

macro_rules! EdgeReference {
    () => {
        deps!();
        # [doc = " Reference to a `StableGraph` edge."] # [derive (Debug)] pub struct EdgeReference < 'a , E : 'a , Ix = DefaultIx > { index : EdgeIndex < Ix > , node : [NodeIndex < Ix > ; 2] , weight : & 'a E , }
    };
}

EdgeReference!()