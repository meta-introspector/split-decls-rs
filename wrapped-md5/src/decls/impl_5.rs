macro_rules! deps {
    () => {
        Md5Core!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl OutputSizeUser for Md5Core { type OutputSize = U16 ; }
    };
}

impl_5!();