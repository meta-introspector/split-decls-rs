macro_rules! deps {
    () => {
        AsyncWriteExt!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < W : AsyncWrite + ? Sized > AsyncWriteExt for W { }
    };
}

impl_300!();