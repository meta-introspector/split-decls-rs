macro_rules! deps {
    () => {
        CompactOptions!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        unsafe impl Sync for CompactOptions { }
    };
}

impl_183!()