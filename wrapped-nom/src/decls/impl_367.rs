macro_rules! deps {
    () => {
        ErrorConvert!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl ErrorConvert < () > for () { fn convert (self) { } }
    };
}

impl_367!()