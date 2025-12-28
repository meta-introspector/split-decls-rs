macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < I : Copy > Error < & I > { # [doc = " Converts `Error<&I>` into `Error<I>` by copying."] pub fn copied (self) -> Error < I > { Error { input : * self . input , code : self . code , } } }
    };
}

impl_17!();