macro_rules! deps {
    () => {
        EdgeIndex!();
        DefaultIx!();
    };
}

macro_rules! Node {
    () => {
        deps!();
        # [doc = " The graph's node type."] # [derive (Debug)] pub struct Node < N , Ix = DefaultIx > { # [doc = " Associated node data."] pub weight : N , # [doc = " Next edge in outgoing and incoming edge lists."] next : [EdgeIndex < Ix > ; 2] , }
    };
}

Node!()