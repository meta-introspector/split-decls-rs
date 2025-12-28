macro_rules! deps {
    () => {
        WriteOptions!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        unsafe impl Sync for WriteOptions { }
    };
}

impl_175!();