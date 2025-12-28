macro_rules! deps {
    () => {
        Undirected!();
        UndirectedAdaptor!();
        EdgeType!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < G > GraphProp for UndirectedAdaptor < G > where G : GraphBase , { type EdgeType = crate :: Undirected ; fn is_directed (& self) -> bool { false } }
    };
}

impl_200!()