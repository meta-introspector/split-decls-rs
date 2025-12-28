macro_rules! deps {
    () => {
        RawRwLock!();
        RwLockReadGuardDetached!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock > RwLockReadGuardDetached < 'a , R > { # [doc = " Separates the data from the [`RwLockReadGuard`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The data must not outlive the detached guard"] pub (crate) unsafe fn detach_from < T > (guard : RwLockReadGuard < 'a , R , T >) -> (Self , & 'a T) { let rwlock = RwLockReadGuard :: rwlock (& ManuallyDrop :: new (guard)) ; let data = unsafe { & * rwlock . data_ptr () } ; let guard = RwLockReadGuardDetached { lock : unsafe { rwlock . raw () } , _marker : PhantomData , } ; (guard , data) } }
    };
}

impl_145!();