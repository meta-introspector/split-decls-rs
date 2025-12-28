macro_rules! deps {
    () => {
        ReadOptions!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        unsafe impl Send for ReadOptions { }
    };
}

impl_169!();