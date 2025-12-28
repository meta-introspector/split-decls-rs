macro_rules! store_in_lru_cache {
    () => {
        pub fn store_in_lru_cache (key : String , value : Vec < u8 >) { if let Ok (mut cache) = LRU_CACHE . lock () { cache . put (key , value) ; } }
    };
}

store_in_lru_cache!();