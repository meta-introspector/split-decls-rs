macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
        RawRwLock!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock > RwLockWriteGuardDetached < 'a , R > { # [doc = " Separates the data from the [`RwLockWriteGuard`]"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The data must not outlive the detached guard"] pub (crate) unsafe fn detach_from < T > (guard : RwLockWriteGuard < 'a , R , T >) -> (Self , & 'a mut T) { let rwlock = RwLockWriteGuard :: rwlock (& ManuallyDrop :: new (guard)) ; let data = unsafe { & mut * rwlock . data_ptr () } ; let guard = RwLockWriteGuardDetached { lock : unsafe { rwlock . raw () } , _marker : PhantomData , } ; (guard , data) } }
    };
}

impl_146!();