macro_rules! deps {
    () => {
        DiGraph!();
    };
}

macro_rules! macro_276 {
    () => {
        deps!();
        impl_graph_traits ! (DiGraph) ;
    };
}

macro_276!()