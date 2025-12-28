macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        Graph!();
    };
}

macro_rules! impl_745 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: EdgeCount for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { # [inline] fn edge_count (& self) -> usize { self . edge_count () } }
    };
}

impl_745!()