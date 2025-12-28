macro_rules! deps {
    () => {
        VisitorResult!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < T > VisitorResult for ControlFlow < T > { type Residual = T ; fn output () -> Self { ControlFlow :: Continue (()) } fn from_residual (residual : Self :: Residual) -> Self { ControlFlow :: Break (residual) } fn from_branch (b : Self) -> Self { b } fn branch (self) -> Self { self } }
    };
}

impl_2!()