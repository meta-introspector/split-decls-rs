macro_rules! deps {
    () => {
        AsyncReadTestExt!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < R > AsyncReadTestExt for R where R : AsyncRead { }
    };
}

impl_64!();