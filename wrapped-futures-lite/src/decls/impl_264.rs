macro_rules! deps {
    () => {
        AsyncReadExt!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < R : AsyncRead + ? Sized > AsyncReadExt for R { }
    };
}

impl_264!();