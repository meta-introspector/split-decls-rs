macro_rules! deps {
    () => {
        MaxRecursionReached!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl std :: error :: Error for MaxRecursionReached { }
    };
}

impl_7!()