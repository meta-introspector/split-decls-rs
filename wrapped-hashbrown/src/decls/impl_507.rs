macro_rules! deps {
    () => {
        IterHash!();
    };
}

macro_rules! impl_507 {
    () => {
        deps!();
        impl < T > FusedIterator for IterHash < '_ , T > { }
    };
}

impl_507!()