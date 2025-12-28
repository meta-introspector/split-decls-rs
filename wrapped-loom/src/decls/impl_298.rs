macro_rules! deps {
    () => {
        Mutex!();
        MutexGuard!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < T : ? Sized > Mutex < T > { # [doc = " Acquires a mutex, blocking the current thread until it is able to do so."] # [track_caller] pub fn lock (& self) -> LockResult < MutexGuard < '_ , T > > { self . object . acquire_lock (location ! ()) ; Ok (MutexGuard { lock : self , data : Some (self . data . lock () . unwrap ()) , }) } # [doc = " Attempts to acquire this lock."] # [doc = ""] # [doc = " If the lock could not be acquired at this time, then `Err` is returned."] # [doc = " Otherwise, an RAII guard is returned. The lock will be unlocked when the"] # [doc = " guard is dropped."] # [doc = ""] # [doc = " This function does not block."] # [track_caller] pub fn try_lock (& self) -> TryLockResult < MutexGuard < '_ , T > > { if self . object . try_acquire_lock (location ! ()) { Ok (MutexGuard { lock : self , data : Some (self . data . lock () . unwrap ()) , }) } else { Err (TryLockError :: WouldBlock) } } # [doc = " Returns a mutable reference to the underlying data."] pub fn get_mut (& mut self) -> LockResult < & mut T > { Ok (self . data . get_mut () . unwrap ()) } }
    };
}

impl_298!()