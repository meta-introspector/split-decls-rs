macro_rules! deps {
    () => {
        AsyncReadExt!();
    };
}

macro_rules! impl_1248 {
    () => {
        deps!();
        impl < R : AsyncRead + ? Sized > AsyncReadExt for R { }
    };
}

impl_1248!()