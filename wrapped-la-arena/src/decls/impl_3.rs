macro_rules! deps {
    () => {
        ArenaMap!();
        Idx!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T , V > std :: ops :: IndexMut < Idx < V > > for ArenaMap < Idx < V > , T > { fn index_mut (& mut self , idx : Idx < V >) -> & mut T { self . v [Self :: to_idx (idx)] . as_mut () . unwrap () } }
    };
}

impl_3!()