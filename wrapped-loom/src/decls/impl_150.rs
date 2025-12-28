macro_rules! deps {
    () => {
        Lazy!();
        StaticKeyId!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl StaticKeyId { fn new < T > (key : & 'static crate :: lazy_static :: Lazy < T >) -> Self { Self (key as * const _ as usize) } }
    };
}

impl_150!();