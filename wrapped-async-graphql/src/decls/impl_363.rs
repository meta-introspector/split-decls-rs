macro_rules! deps {
    () => {
        NoCacheImpl!();
        CacheStorage!();
        NoCache!();
        CacheFactory!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl CacheFactory for NoCache { fn create < K , V > (& self) -> Box < dyn CacheStorage < Key = K , Value = V > > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { Box :: new (NoCacheImpl { _mark1 : PhantomData , _mark2 : PhantomData , }) } }
    };
}

impl_363!();