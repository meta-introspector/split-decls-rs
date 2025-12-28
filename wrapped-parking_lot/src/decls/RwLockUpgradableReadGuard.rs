macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! RwLockUpgradableReadGuard {
    () => {
        deps!();
        # [doc = " RAII structure used to release the upgradable read access of a lock when"] # [doc = " dropped."] pub type RwLockUpgradableReadGuard < 'a , T > = lock_api :: RwLockUpgradableReadGuard < 'a , RawRwLock , T > ;
    };
}

RwLockUpgradableReadGuard!()