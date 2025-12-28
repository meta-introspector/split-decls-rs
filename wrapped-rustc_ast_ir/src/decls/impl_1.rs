macro_rules! deps {
    () => {
        VisitorResult!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl VisitorResult for () { # [cfg (feature = "nightly")] type Residual = ! ; # [cfg (not (feature = "nightly"))] type Residual = core :: convert :: Infallible ; fn output () -> Self { } fn from_residual (_ : Self :: Residual) -> Self { } fn from_branch (_ : ControlFlow < Self :: Residual >) -> Self { } fn branch (self) -> ControlFlow < Self :: Residual > { ControlFlow :: Continue (()) } }
    };
}

impl_1!()