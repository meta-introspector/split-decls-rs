macro_rules! deps {
    () => {
        ArcRwLockWriteGuard!();
        ArcRwLockReadGuard!();
        RwLockWriteGuard!();
        RawRwLockDowngrade!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockDowngrade , T : ? Sized > ArcRwLockWriteGuard < R , T > { # [doc = " Atomically downgrades a write lock into a read lock without allowing any"] # [doc = " writers to take exclusive access of the lock in the meantime."] # [doc = ""] # [doc = " This is functionally equivalent to the `downgrade` method on [`RwLockWriteGuard`]."] # [track_caller] pub fn downgrade (s : Self) -> ArcRwLockReadGuard < R , T > { unsafe { s . rwlock . raw . downgrade () ; } let s = ManuallyDrop :: new (s) ; let rwlock = unsafe { ptr :: read (& s . rwlock) } ; ArcRwLockReadGuard { rwlock , marker : PhantomData , } } }
    };
}

impl_152!()