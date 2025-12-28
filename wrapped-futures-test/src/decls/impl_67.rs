macro_rules! deps {
    () => {
        AsyncWriteTestExt!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < W > AsyncWriteTestExt for W where W : AsyncWrite { }
    };
}

impl_67!();