macro_rules! deps {
    () => {
        CacheError!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl CacheError { pub (crate) fn too_many_cache_clears () -> CacheError { CacheError (()) } pub (crate) fn bad_efficiency () -> CacheError { CacheError (()) } }
    };
}

impl_262!();