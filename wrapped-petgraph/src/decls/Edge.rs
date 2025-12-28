macro_rules! deps {
    () => {
        EdgeIndex!();
        DefaultIx!();
        NodeIndex!();
    };
}

macro_rules! Edge {
    () => {
        deps!();
        # [doc = " The graph's edge type."] # [derive (Debug)] pub struct Edge < E , Ix = DefaultIx > { # [doc = " Associated edge data."] pub weight : E , # [doc = " Next edge in outgoing and incoming edge lists."] next : [EdgeIndex < Ix > ; 2] , # [doc = " Start and End node index"] node : [NodeIndex < Ix > ; 2] , }
    };
}

Edge!();