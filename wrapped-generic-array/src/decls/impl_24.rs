macro_rules! deps {
    () => {
        GenericArrayImplOdd!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : Copy , U : Copy > Copy for GenericArrayImplOdd < T , U > { }
    };
}

impl_24!()