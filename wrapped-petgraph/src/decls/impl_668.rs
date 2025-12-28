macro_rules! deps {
    () => {
        Direction!();
        EdgeIndex!();
        Node!();
        IndexType!();
    };
}

macro_rules! impl_668 {
    () => {
        deps!();
        impl < N , Ix : IndexType > Node < N , Ix > { # [doc = " Accessor for data structure internals: the first edge in the given direction."] pub fn next_edge (& self , dir : Direction) -> EdgeIndex < Ix > { self . next [dir . index ()] } }
    };
}

impl_668!()