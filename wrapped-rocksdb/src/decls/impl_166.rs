macro_rules! deps {
    () => {
        FlushOptions!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        unsafe impl Send for FlushOptions { }
    };
}

impl_166!();