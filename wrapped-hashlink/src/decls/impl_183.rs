macro_rules! deps {
    () => {
        LruCache!();
        LinkedHashMap!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < K : Eq + Hash , V > LruCache < K , V > { # [inline] pub fn new (capacity : usize) -> Self { LruCache { map : LinkedHashMap :: new () , max_size : capacity , } } # [doc = " Create a new unbounded `LruCache` that does not automatically evict entries."] # [doc = ""] # [doc = " A simple convenience method that is equivalent to `LruCache::new(usize::MAX)`"] # [inline] pub fn new_unbounded () -> Self { LruCache :: new (usize :: MAX) } }
    };
}

impl_183!();