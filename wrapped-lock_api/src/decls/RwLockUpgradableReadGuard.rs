macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        RwLock!();
    };
}

macro_rules! RwLockUpgradableReadGuard {
    () => {
        deps!();
        # [doc = " RAII structure used to release the upgradable read access of a lock when"] # [doc = " dropped."] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct RwLockUpgradableReadGuard < 'a , R : RawRwLockUpgrade , T : ? Sized > { rwlock : & 'a RwLock < R , T > , marker : PhantomData < (& 'a T , R :: GuardMarker) > , }
    };
}

RwLockUpgradableReadGuard!();