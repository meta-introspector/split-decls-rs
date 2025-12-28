macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < G : Visitable + DataMap > DataMap for Acyclic < G > { fn node_weight (& self , id : Self :: NodeId) -> Option < & Self :: NodeWeight > { self . inner () . node_weight (id) } fn edge_weight (& self , id : Self :: EdgeId) -> Option < & Self :: EdgeWeight > { self . inner () . edge_weight (id) } }
    };
}

impl_265!()