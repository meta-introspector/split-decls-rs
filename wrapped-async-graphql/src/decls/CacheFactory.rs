macro_rules! deps {
    () => {
        CacheStorage!();
    };
}

macro_rules! CacheFactory {
    () => {
        deps!();
        # [doc = " Factory for creating cache storage."] pub trait CacheFactory : Send + Sync + 'static { # [doc = " Create a cache storage."] # [doc = ""] # [doc = " TODO: When GAT is stable, this memory allocation can be optimized away."] fn create < K , V > (& self) -> Box < dyn CacheStorage < Key = K , Value = V > > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static ; }
    };
}

CacheFactory!()