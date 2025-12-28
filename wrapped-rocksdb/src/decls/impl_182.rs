macro_rules! deps {
    () => {
        CacheWrapper!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        unsafe impl Sync for CacheWrapper { }
    };
}

impl_182!();