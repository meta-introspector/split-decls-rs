macro_rules! deps {
    () => {
        HashMapCache!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl < S : Send + Sync + BuildHasher + Default + 'static > HashMapCache < S > { # [doc = " Use specified `S: BuildHasher` to create a `HashMap` cache."] pub fn new () -> Self { Self { _mark : PhantomData } } }
    };
}

impl_367!()