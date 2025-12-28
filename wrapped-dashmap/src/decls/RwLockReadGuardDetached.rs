macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! RwLockReadGuardDetached {
    () => {
        deps!();
        # [doc = " A [`RwLockReadGuard`], without the data"] pub (crate) struct RwLockReadGuardDetached < 'a , R : RawRwLock > { lock : & 'a R , _marker : PhantomData < R :: GuardMarker > , }
    };
}

RwLockReadGuardDetached!();