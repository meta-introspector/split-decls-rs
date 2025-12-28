macro_rules! deps {
    () => {
        GenericArrayImplEven!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T : Copy , U : Copy > Copy for GenericArrayImplEven < T , U > { }
    };
}

impl_23!()