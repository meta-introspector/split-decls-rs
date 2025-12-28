macro_rules! deps {
    () => {
        Matching!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl < G > Matching < G > where G : GraphBase , { fn new (graph : G , mate : Vec < Option < G :: NodeId > > , n_edges : usize) -> Self { Self { graph , mate , n_edges , } } }
    };
}

impl_398!()