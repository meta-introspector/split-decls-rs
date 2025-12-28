macro_rules! deps {
    () => {
        LruCacheOptions!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        unsafe impl Send for LruCacheOptions { }
    };
}

impl_165!()