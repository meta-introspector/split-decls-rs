macro_rules! HashMapCacheImpl {
    () => {
        struct HashMapCacheImpl < K , V , S > (HashMap < K , V , S >) ;
    };
}

HashMapCacheImpl!();