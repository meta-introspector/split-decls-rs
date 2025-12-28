macro_rules! STATEMENT_CACHE_DEFAULT_CAPACITY {
    () => {
        # [cfg (feature = "cache")] const STATEMENT_CACHE_DEFAULT_CAPACITY : usize = 16 ;
    };
}

STATEMENT_CACHE_DEFAULT_CAPACITY!()