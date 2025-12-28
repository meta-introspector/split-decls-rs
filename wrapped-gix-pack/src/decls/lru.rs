macro_rules! deps {
    () => {
        DecodeEntry!();
    };
}

macro_rules! lru {
    () => {
        deps!();
        # [doc = " Various implementations of [`DecodeEntry`] using least-recently-used algorithms."] # [cfg (any (feature = "pack-cache-lru-dynamic" , feature = "pack-cache-lru-static"))] pub mod lru ;
    };
}

lru!();