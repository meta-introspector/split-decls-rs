macro_rules! deps {
    () => {
        ToGraph6!();
        Graph!();
        IndexType!();
        Undirected!();
    };
}

macro_rules! impl_618 {
    () => {
        deps!();
        impl < N , E , Ix : IndexType > ToGraph6 for Graph < N , E , Undirected , Ix > { fn graph6_string (& self) -> String { get_graph6_representation (self) } }
    };
}

impl_618!()