macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < G : Visitable + EdgeIndexable > EdgeIndexable for Acyclic < G > { fn edge_bound (& self) -> usize { self . inner () . edge_bound () } fn to_index (& self , a : Self :: EdgeId) -> usize { self . inner () . to_index (a) } fn from_index (& self , i : usize) -> Self :: EdgeId { self . inner () . from_index (i) } }
    };
}

impl_268!();