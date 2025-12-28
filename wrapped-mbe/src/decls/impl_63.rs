macro_rules! deps {
    () => {
        ValueResult!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T : Default , E > From < Result < T , E > > for ValueResult < T , E > { fn from (result : Result < T , E >) -> Self { result . map_or_else (Self :: only_err , Self :: ok) } }
    };
}

impl_63!();