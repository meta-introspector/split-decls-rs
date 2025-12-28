macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < G : Visitable + EdgeCount > EdgeCount for Acyclic < G > { fn edge_count (& self) -> usize { self . inner () . edge_count () } }
    };
}

impl_267!()