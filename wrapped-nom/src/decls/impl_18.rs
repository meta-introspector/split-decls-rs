macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < I : Copy > Error < & mut I > { # [doc = " Converts `Error<&mut I>` into `Error<I>` by copying."] pub fn copied (self) -> Error < I > { Error { input : * self . input , code : self . code , } } }
    };
}

impl_18!()