macro_rules! deps {
    () => {
        IndexType!();
        Edge!();
        Direction!();
        EdgeIndex!();
        NodeIndex!();
    };
}

macro_rules! impl_671 {
    () => {
        deps!();
        impl < E , Ix : IndexType > Edge < E , Ix > { # [doc = " Accessor for data structure internals: the next edge for the given direction."] pub fn next_edge (& self , dir : Direction) -> EdgeIndex < Ix > { self . next [dir . index ()] } # [doc = " Return the source node index."] pub fn source (& self) -> NodeIndex < Ix > { self . node [0] } # [doc = " Return the target node index."] pub fn target (& self) -> NodeIndex < Ix > { self . node [1] } }
    };
}

impl_671!();