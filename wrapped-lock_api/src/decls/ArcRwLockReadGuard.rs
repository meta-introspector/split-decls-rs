macro_rules! deps {
    () => {
        RwLock!();
        RwLockReadGuard!();
        RawRwLock!();
    };
}

macro_rules! ArcRwLockReadGuard {
    () => {
        deps!();
        # [doc = " An RAII rwlock guard returned by the `Arc` locking operations on `RwLock`."] # [doc = ""] # [doc = " This is similar to the `RwLockReadGuard` struct, except instead of using a reference to unlock the `RwLock`"] # [doc = " it uses an `Arc<RwLock>`. This has several advantages, most notably that it has an `'static` lifetime."] # [cfg (feature = "arc_lock")] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct ArcRwLockReadGuard < R : RawRwLock , T : ? Sized > { rwlock : Arc < RwLock < R , T > > , marker : PhantomData < R :: GuardMarker > , }
    };
}

ArcRwLockReadGuard!()