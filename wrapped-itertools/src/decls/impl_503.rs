macro_rules! deps {
    () => {
        TupleCollect!();
        HomogeneousTuple!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl < T : TupleCollect > HomogeneousTuple for T { }
    };
}

impl_503!();