macro_rules! deps {
    () => {
        StableGraph!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_863 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: EdgeCount for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { # [inline] fn edge_count (& self) -> usize { self . edge_count () } }
    };
}

impl_863!()