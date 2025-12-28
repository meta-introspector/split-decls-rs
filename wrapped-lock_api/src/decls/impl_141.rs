macro_rules! deps {
    () => {
        RwLockWriteGuard!();
        RawRwLockDowngrade!();
        RwLockReadGuard!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockDowngrade + 'a , T : ? Sized + 'a > RwLockWriteGuard < 'a , R , T > { # [doc = " Atomically downgrades a write lock into a read lock without allowing any"] # [doc = " writers to take exclusive access of the lock in the meantime."] # [doc = ""] # [doc = " Note that if there are any writers currently waiting to take the lock"] # [doc = " then other readers may not be able to acquire the lock even if it was"] # [doc = " downgraded."] # [track_caller] pub fn downgrade (s : Self) -> RwLockReadGuard < 'a , R , T > { unsafe { s . rwlock . raw . downgrade () ; } let rwlock = s . rwlock ; mem :: forget (s) ; RwLockReadGuard { rwlock , marker : PhantomData , } } }
    };
}

impl_141!()