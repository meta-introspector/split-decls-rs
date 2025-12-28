macro_rules! deps {
    () => {
        CachePoolFn!();
        PoolGuard!();
        Cache!();
    };
}

macro_rules! CachePoolGuard {
    () => {
        deps!();
        # [doc = " Same as above, but for the guard returned by a pool."] type CachePoolGuard < 'a > = PoolGuard < 'a , Cache , CachePoolFn > ;
    };
}

CachePoolGuard!()