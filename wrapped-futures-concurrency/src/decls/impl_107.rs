macro_rules! deps {
    () => {
        Try!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < B , C > Try for ControlFlow < B , C > { private_impl ! { } type Output = C ; type Residual = ControlFlow < B , Infallible > ; fn from_output (output : Self :: Output) -> Self { Continue (output) } fn from_residual (residual : Self :: Residual) -> Self { match residual { Break (b) => Break (b) , Continue (_) => unreachable ! () , } } fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > { match self { Continue (c) => Continue (c) , Break (b) => Break (Break (b)) , } } }
    };
}

impl_107!()