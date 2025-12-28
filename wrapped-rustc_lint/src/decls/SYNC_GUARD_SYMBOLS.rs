macro_rules! SYNC_GUARD_SYMBOLS {
    () => {
        const SYNC_GUARD_SYMBOLS : [Symbol ; 3] = [rustc_span :: sym :: MutexGuard , rustc_span :: sym :: RwLockReadGuard , rustc_span :: sym :: RwLockWriteGuard ,] ;
    };
}

SYNC_GUARD_SYMBOLS!()