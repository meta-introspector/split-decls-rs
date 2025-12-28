macro_rules! deps {
    () => {
        GenericArrayImplOdd!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < T : Copy , U : Copy > Copy for GenericArrayImplOdd < T , U > { }
    };
}

impl_170!();