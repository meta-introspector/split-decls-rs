macro_rules! deps {
    () => {
        RetryQuadraticError!();
        RetryError!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl From < RetryQuadraticError > for RetryError { fn from (err : RetryQuadraticError) -> RetryError { RetryError :: Quadratic (err) } }
    };
}

impl_314!()