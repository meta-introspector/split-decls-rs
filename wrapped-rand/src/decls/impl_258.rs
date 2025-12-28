macro_rules! deps {
    () => {
        ThreadRng!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl CryptoRng for ThreadRng { }
    };
}

impl_258!()