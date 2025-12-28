macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl < G : Visitable > Deref for Acyclic < G > { type Target = G ; fn deref (& self) -> & Self :: Target { & self . graph } }
    };
}

impl_262!()