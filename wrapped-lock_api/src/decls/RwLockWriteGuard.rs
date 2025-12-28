macro_rules! deps {
    () => {
        RwLock!();
        RawRwLock!();
    };
}

macro_rules! RwLockWriteGuard {
    () => {
        deps!();
        # [doc = " RAII structure used to release the exclusive write access of a lock when"] # [doc = " dropped."] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct RwLockWriteGuard < 'a , R : RawRwLock , T : ? Sized > { rwlock : & 'a RwLock < R , T > , marker : PhantomData < (& 'a mut T , R :: GuardMarker) > , }
    };
}

RwLockWriteGuard!()