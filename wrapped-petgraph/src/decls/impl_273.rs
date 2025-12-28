macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < G : Visitable + NodeIndexable > NodeIndexable for Acyclic < G > { fn node_bound (& self) -> usize { self . inner () . node_bound () } fn to_index (& self , a : Self :: NodeId) -> usize { self . inner () . to_index (a) } fn from_index (& self , i : usize) -> Self :: NodeId { self . inner () . from_index (i) } }
    };
}

impl_273!();