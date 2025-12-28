macro_rules! deps {
    () => {
        FromHexError!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl core :: error :: Error for FromHexError { }
    };
}

impl_2!();