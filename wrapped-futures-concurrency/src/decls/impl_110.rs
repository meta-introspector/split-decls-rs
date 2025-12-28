macro_rules! deps {
    () => {
        Try!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < T , E > Try for Poll < Result < T , E > > { private_impl ! { } type Output = Poll < T > ; type Residual = Result < Infallible , E > ; fn from_output (output : Self :: Output) -> Self { output . map (Ok) } fn from_residual (residual : Self :: Residual) -> Self { match residual { Err (e) => Poll :: Ready (Err (e)) , Ok (_) => unreachable ! () , } } fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > { match self { Poll :: Pending => Continue (Poll :: Pending) , Poll :: Ready (Ok (c)) => Continue (Poll :: Ready (c)) , Poll :: Ready (Err (e)) => Break (Err (e)) , } } }
    };
}

impl_110!();