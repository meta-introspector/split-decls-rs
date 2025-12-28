macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < G : Visitable + NodeCount > NodeCount for Acyclic < G > { fn node_count (& self) -> usize { self . inner () . node_count () } }
    };
}

impl_272!();