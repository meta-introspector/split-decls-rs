macro_rules! deps {
    () => {
        Reversed!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < G : Visitable > Visitable for Reversed < G > { type Map = G :: Map ; fn visit_map (& self) -> G :: Map { self . 0 . visit_map () } fn reset_map (& self , map : & mut Self :: Map) { self . 0 . reset_map (map) ; } }
    };
}

impl_176!()