macro_rules! deps {
    () => {
        PoolGuard!();
        Cache!();
        CachePoolFn!();
    };
}

macro_rules! CachePoolGuard {
    () => {
        deps!();
        # [doc = " Same as above, but for the guard returned by a pool."] type CachePoolGuard < 'a > = PoolGuard < 'a , Cache , CachePoolFn > ;
    };
}

CachePoolGuard!();