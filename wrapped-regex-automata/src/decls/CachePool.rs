macro_rules! deps {
    () => {
        Cache!();
        CachePoolFn!();
        Pool!();
    };
}

macro_rules! CachePool {
    () => {
        deps!();
        # [doc = " A type alias for our pool of meta::Cache that fixes the type parameters to"] # [doc = " what we use for the meta regex below."] type CachePool = Pool < Cache , CachePoolFn > ;
    };
}

CachePool!();