macro_rules! deps {
    () => {
        RwLock!();
        RawRwLockRecursiveTimed!();
        RwLockReadGuard!();
        ArcRwLockReadGuard!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < R : RawRwLockRecursiveTimed , T : ? Sized > RwLock < R , T > { # [doc = " Attempts to acquire this `RwLock` with shared read access until a timeout"] # [doc = " is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " `None` is returned. Otherwise, an RAII guard is returned which will"] # [doc = " release the shared access when it is dropped."] # [doc = ""] # [doc = " This method is guaranteed to succeed without blocking if another read"] # [doc = " lock is held at the time of the call. See the documentation for"] # [doc = " `read_recursive` for details."] # [inline] # [track_caller] pub fn try_read_recursive_for (& self , timeout : R :: Duration ,) -> Option < RwLockReadGuard < '_ , R , T > > { if self . raw . try_lock_shared_recursive_for (timeout) { Some (unsafe { self . make_read_guard_unchecked () }) } else { None } } # [doc = " Attempts to acquire this `RwLock` with shared read access until a timeout"] # [doc = " is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " `None` is returned. Otherwise, an RAII guard is returned which will"] # [doc = " release the shared access when it is dropped."] # [inline] # [track_caller] pub fn try_read_recursive_until (& self , timeout : R :: Instant ,) -> Option < RwLockReadGuard < '_ , R , T > > { if self . raw . try_lock_shared_recursive_until (timeout) { Some (unsafe { self . make_read_guard_unchecked () }) } else { None } } # [doc = " Attempts to lock this `RwLock` with read access until a timeout is reached, through an `Arc`."] # [doc = ""] # [doc = " This method is similar to the `try_read_recursive_for` method; however, it requires the `RwLock` to be"] # [doc = " inside of an `Arc` and the resulting read guard has no lifetime requirements."] # [cfg (feature = "arc_lock")] # [inline] # [track_caller] pub fn try_read_arc_recursive_for (self : & Arc < Self > , timeout : R :: Duration ,) -> Option < ArcRwLockReadGuard < R , T > > { if self . raw . try_lock_shared_recursive_for (timeout) { Some (unsafe { self . make_arc_read_guard_unchecked () }) } else { None } } # [doc = " Attempts to lock this `RwLock` with read access until a timeout is reached, through an `Arc`."] # [doc = ""] # [doc = " This method is similar to the `try_read_recursive_until` method; however, it requires the `RwLock` to be"] # [doc = " inside of an `Arc` and the resulting read guard has no lifetime requirements."] # [cfg (feature = "arc_lock")] # [inline] # [track_caller] pub fn try_read_arc_recursive_until (self : & Arc < Self > , timeout : R :: Instant ,) -> Option < ArcRwLockReadGuard < R , T > > { if self . raw . try_lock_shared_recursive_until (timeout) { Some (unsafe { self . make_arc_read_guard_unchecked () }) } else { None } } }
    };
}

impl_116!()