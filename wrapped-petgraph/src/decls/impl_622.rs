macro_rules! deps {
    () => {
        IndexType!();
        Csr!();
        Undirected!();
        ToGraph6!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl < N , E , Ix : IndexType > ToGraph6 for Csr < N , E , Undirected , Ix > { fn graph6_string (& self) -> String { get_graph6_representation (self) } }
    };
}

impl_622!()