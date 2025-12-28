macro_rules! deps {
    () => {
        CacheKey!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl Eq for CacheKey { }
    };
}

impl_113!();