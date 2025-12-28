macro_rules! deps {
    () => {
        RetryQuadraticError!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl RetryQuadraticError { pub (crate) fn new () -> RetryQuadraticError { RetryQuadraticError (()) } }
    };
}

impl_311!()