macro_rules! deps {
    () => {
        Try!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < T > Try for Option < T > { private_impl ! { } type Output = T ; type Residual = Option < Infallible > ; fn from_output (output : Self :: Output) -> Self { Some (output) } fn from_residual (residual : Self :: Residual) -> Self { match residual { None => None , Some (_) => unreachable ! () , } } fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > { match self { Some (c) => Continue (c) , None => Break (None) , } } }
    };
}

impl_108!()