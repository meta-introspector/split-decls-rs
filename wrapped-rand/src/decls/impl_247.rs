macro_rules! deps {
    () => {
        StdRng!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl CryptoRng for StdRng { }
    };
}

impl_247!();