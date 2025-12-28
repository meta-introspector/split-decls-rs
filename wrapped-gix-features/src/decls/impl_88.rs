macro_rules! deps {
    () => {
        Decompress!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        unsafe impl Send for Decompress { }
    };
}

impl_88!();