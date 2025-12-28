macro_rules! deps {
    () => {
        RwLockUpgradableReadGuard!();
        RwLock!();
        RawRwLockUpgrade!();
    };
}

macro_rules! ArcRwLockUpgradableReadGuard {
    () => {
        deps!();
        # [doc = " An RAII rwlock guard returned by the `Arc` locking operations on `RwLock`."] # [doc = " This is similar to the `RwLockUpgradableReadGuard` struct, except instead of using a reference to unlock the"] # [doc = " `RwLock` it uses an `Arc<RwLock>`. This has several advantages, most notably that it has an `'static`"] # [doc = " lifetime."] # [cfg (feature = "arc_lock")] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct ArcRwLockUpgradableReadGuard < R : RawRwLockUpgrade , T : ? Sized > { rwlock : Arc < RwLock < R , T > > , marker : PhantomData < R :: GuardMarker > , }
    };
}

ArcRwLockUpgradableReadGuard!()