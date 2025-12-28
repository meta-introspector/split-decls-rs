macro_rules! GLOBAL_POOL_ALREADY_INITIALIZED {
    () => {
        const GLOBAL_POOL_ALREADY_INITIALIZED : & str = "The global thread pool has already been initialized." ;
    };
}

GLOBAL_POOL_ALREADY_INITIALIZED!();