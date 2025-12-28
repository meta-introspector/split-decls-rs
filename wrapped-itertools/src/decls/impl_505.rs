macro_rules! deps {
    () => {
        TupleBuffer!();
        HomogeneousTuple!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl < T > TupleBuffer < T > where T : HomogeneousTuple , { fn new (buf : T :: Buffer) -> Self { Self { cur : 0 , buf } } }
    };
}

impl_505!()