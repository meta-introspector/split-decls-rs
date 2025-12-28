macro_rules! deps {
    () => {
        CacheWrapper!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        unsafe impl Send for CacheWrapper { }
    };
}

impl_171!();