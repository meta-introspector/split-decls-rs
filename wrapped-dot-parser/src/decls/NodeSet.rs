macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! NodeSet {
    () => {
        deps!();
        # [doc = " A set of `Node`s."] # [derive (Debug , Clone)] pub struct NodeSet < A > { # [doc = " The set of nodes in the NodeSet. They are indexed by their identifier."] # [doc = " Note that this field being public is experimental."] pub set : HashMap < String , Node < A > > , }
    };
}

NodeSet!()