macro_rules! deps {
    () => {
        ReadCacheRange!();
        ReadCacheOps!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a , R : ReadCacheOps > Copy for ReadCacheRange < 'a , R > { }
    };
}

impl_93!();