macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! RwLockWriteGuard {
    () => {
        deps!();
        # [doc = " RAII structure used to release the exclusive write access of a lock when"] # [doc = " dropped."] pub type RwLockWriteGuard < 'a , T > = lock_api :: RwLockWriteGuard < 'a , RawRwLock , T > ;
    };
}

RwLockWriteGuard!()