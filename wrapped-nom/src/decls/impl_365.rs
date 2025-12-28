macro_rules! deps {
    () => {
        ErrorConvert!();
        Error!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl < I > ErrorConvert < error :: Error < I > > for error :: Error < (I , usize) > { fn convert (self) -> error :: Error < I > { error :: Error { input : self . input . 0 , code : self . code , } } }
    };
}

impl_365!();