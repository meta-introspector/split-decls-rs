macro_rules! deps {
    () => {
        FlushOptions!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        unsafe impl Sync for FlushOptions { }
    };
}

impl_177!();