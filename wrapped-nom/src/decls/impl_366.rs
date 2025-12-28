macro_rules! deps {
    () => {
        ErrorConvert!();
        Error!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl < I > ErrorConvert < error :: Error < (I , usize) > > for error :: Error < I > { fn convert (self) -> error :: Error < (I , usize) > { error :: Error { input : (self . input , 0) , code : self . code , } } }
    };
}

impl_366!();