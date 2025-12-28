macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < G : Visitable + NodeCompactIndexable > NodeCompactIndexable for Acyclic < G > { }
    };
}

impl_271!();