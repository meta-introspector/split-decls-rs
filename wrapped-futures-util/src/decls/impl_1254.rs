macro_rules! deps {
    () => {
        AsyncBufReadExt!();
    };
}

macro_rules! impl_1254 {
    () => {
        deps!();
        impl < R : AsyncBufRead + ? Sized > AsyncBufReadExt for R { }
    };
}

impl_1254!()