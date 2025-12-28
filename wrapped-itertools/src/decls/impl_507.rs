macro_rules! deps {
    () => {
        HomogeneousTuple!();
        TupleBuffer!();
    };
}

macro_rules! impl_507 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for TupleBuffer < T > where T : HomogeneousTuple { }
    };
}

impl_507!();