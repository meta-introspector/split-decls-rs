macro_rules! deps {
    () => {
        FromUtf16Error!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl core :: error :: Error for FromUtf16Error { }
    };
}

impl_222!()