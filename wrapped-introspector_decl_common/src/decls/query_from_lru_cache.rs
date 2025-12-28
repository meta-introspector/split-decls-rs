macro_rules! query_from_lru_cache {
    () => {
        pub fn query_from_lru_cache (key : & str) -> Option < Vec < u8 > > { if let Ok (mut cache) = LRU_CACHE . lock () { cache . get (key) . cloned () } else { None } }
    };
}

query_from_lru_cache!()