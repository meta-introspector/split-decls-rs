macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! RwLockWriteGuardDetached {
    () => {
        deps!();
        # [doc = " A [`RwLockWriteGuard`], without the data"] pub (crate) struct RwLockWriteGuardDetached < 'a , R : RawRwLock > { lock : & 'a R , _marker : PhantomData < R :: GuardMarker > , }
    };
}

RwLockWriteGuardDetached!();