macro_rules! deps {
    () => {
        GenericArrayImplEven!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < T : Copy , U : Copy > Copy for GenericArrayImplEven < T , U > { }
    };
}

impl_169!()