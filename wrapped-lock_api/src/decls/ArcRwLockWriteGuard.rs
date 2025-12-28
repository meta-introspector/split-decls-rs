macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
        RwLockWriteGuard!();
    };
}

macro_rules! ArcRwLockWriteGuard {
    () => {
        deps!();
        # [doc = " An RAII rwlock guard returned by the `Arc` locking operations on `RwLock`."] # [doc = " This is similar to the `RwLockWriteGuard` struct, except instead of using a reference to unlock the `RwLock`"] # [doc = " it uses an `Arc<RwLock>`. This has several advantages, most notably that it has an `'static` lifetime."] # [cfg (feature = "arc_lock")] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct ArcRwLockWriteGuard < R : RawRwLock , T : ? Sized > { rwlock : Arc < RwLock < R , T > > , marker : PhantomData < R :: GuardMarker > , }
    };
}

ArcRwLockWriteGuard!();