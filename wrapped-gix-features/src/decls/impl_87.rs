macro_rules! deps {
    () => {
        Decompress!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        unsafe impl Sync for Decompress { }
    };
}

impl_87!()