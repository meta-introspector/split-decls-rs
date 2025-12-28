macro_rules! deps {
    () => {
        RetryQuadraticError!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for RetryQuadraticError { }
    };
}

impl_312!();