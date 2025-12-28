macro_rules! deps {
    () => {
        ArenaMap!();
        Idx!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < T , V > std :: ops :: Index < Idx < V > > for ArenaMap < Idx < V > , T > { type Output = T ; fn index (& self , idx : Idx < V >) -> & T { self . v [Self :: to_idx (idx)] . as_ref () . unwrap () } }
    };
}

impl_2!();