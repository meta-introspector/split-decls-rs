macro_rules! deps {
    () => {
        CompactOptions!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        unsafe impl Send for CompactOptions { }
    };
}

impl_172!()