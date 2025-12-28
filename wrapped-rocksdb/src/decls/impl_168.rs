macro_rules! deps {
    () => {
        CuckooTableOptions!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        unsafe impl Send for CuckooTableOptions { }
    };
}

impl_168!();