macro_rules! deps {
    () => {
        AsyncBufReadExt!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl < R : AsyncBufRead + ? Sized > AsyncBufReadExt for R { }
    };
}

impl_247!()