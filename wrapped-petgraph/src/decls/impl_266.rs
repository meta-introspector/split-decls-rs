macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < G : Visitable + DataMapMut > DataMapMut for Acyclic < G > { fn node_weight_mut (& mut self , id : Self :: NodeId) -> Option < & mut Self :: NodeWeight > { self . inner_mut () . node_weight_mut (id) } fn edge_weight_mut (& mut self , id : Self :: EdgeId) -> Option < & mut Self :: EdgeWeight > { self . inner_mut () . edge_weight_mut (id) } }
    };
}

impl_266!();