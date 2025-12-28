macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! RwLockReadGuard {
    () => {
        deps!();
        # [doc = " RAII structure used to release the shared read access of a lock when"] # [doc = " dropped."] pub type RwLockReadGuard < 'a , T > = lock_api :: RwLockReadGuard < 'a , RawRwLock , T > ;
    };
}

RwLockReadGuard!();