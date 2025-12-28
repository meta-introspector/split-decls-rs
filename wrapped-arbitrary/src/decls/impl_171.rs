macro_rules! deps {
    () => {
        Error!();
        MaxRecursionReached!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl std :: error :: Error for MaxRecursionReached { }
    };
}

impl_171!()