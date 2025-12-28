macro_rules! deps {
    () => {
        LruCacheOptions!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        unsafe impl Sync for LruCacheOptions { }
    };
}

impl_176!();