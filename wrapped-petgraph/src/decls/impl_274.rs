macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < G : Visitable > Visitable for Acyclic < G > { type Map = G :: Map ; fn visit_map (& self) -> Self :: Map { self . inner () . visit_map () } fn reset_map (& self , map : & mut Self :: Map) { self . inner () . reset_map (map) } }
    };
}

impl_274!()