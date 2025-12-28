macro_rules! deps {
    () => {
        Graph!();
        ToGraph6!();
        Undirected!();
        IndexType!();
    };
}

macro_rules! impl_618 {
    () => {
        deps!();
        impl < N , E , Ix : IndexType > ToGraph6 for Graph < N , E , Undirected , Ix > { fn graph6_string (& self) -> String { get_graph6_representation (self) } }
    };
}

impl_618!();