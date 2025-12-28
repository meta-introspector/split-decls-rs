macro_rules! deps {
    () => {
        CuckooTableOptions!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        unsafe impl Sync for CuckooTableOptions { }
    };
}

impl_179!()