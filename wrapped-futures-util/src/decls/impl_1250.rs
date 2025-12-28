macro_rules! deps {
    () => {
        AsyncWriteExt!();
    };
}

macro_rules! impl_1250 {
    () => {
        deps!();
        impl < W : AsyncWrite + ? Sized > AsyncWriteExt for W { }
    };
}

impl_1250!();