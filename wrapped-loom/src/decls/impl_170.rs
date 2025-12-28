macro_rules! deps {
    () => {
        LocalKeyId!();
        LocalKey!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl LocalKeyId { fn new < T > (key : & 'static crate :: thread :: LocalKey < T >) -> Self { Self (key as * const _ as usize) } }
    };
}

impl_170!();