macro_rules! deps {
    () => {
        ArticulationPointTracker!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl ArticulationPointTracker { fn new (graph_size : usize) -> Self { Self { visited : FixedBitSet :: with_capacity (graph_size) , low : vec ! [usize :: MAX ; graph_size] , disc : vec ! [usize :: MAX ; graph_size] , parent : vec ! [usize :: MAX ; graph_size] , articulation_points : HashSet :: with_capacity (graph_size) , time : 0 , } } }
    };
}

impl_327!();