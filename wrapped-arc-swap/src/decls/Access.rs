macro_rules! Access {
    () => {
        # [doc = " Generalization of caches providing access to `T`."] # [doc = ""] # [doc = " This abstracts over all kinds of caches that can provide a cheap access to values of type `T`."] # [doc = " This is useful in cases where some code doesn't care if the `T` is the whole structure or just"] # [doc = " a part of it."] # [doc = ""] # [doc = " See the example at [`Cache::map`]."] pub trait Access < T > { # [doc = " Loads the value from cache."] # [doc = ""] # [doc = " This revalidates the value in the cache, then provides the access to the cached value."] fn load (& mut self) -> & T ; }
    };
}

Access!()