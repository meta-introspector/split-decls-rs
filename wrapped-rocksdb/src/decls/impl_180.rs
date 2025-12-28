macro_rules! deps {
    () => {
        ReadOptions!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        unsafe impl Sync for ReadOptions { }
    };
}

impl_180!();