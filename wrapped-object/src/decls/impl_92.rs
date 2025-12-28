macro_rules! deps {
    () => {
        ReadCacheRange!();
        ReadCacheOps!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'a , R : ReadCacheOps > Clone for ReadCacheRange < 'a , R > { fn clone (& self) -> Self { * self } }
    };
}

impl_92!()