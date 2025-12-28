macro_rules! deps {
    () => {
        StableDiGraph!();
    };
}

macro_rules! macro_277 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl_graph_traits ! (StableDiGraph) ;
    };
}

macro_277!()