macro_rules! deps {
    () => {
        ArcRwLockUpgradableReadGuard!();
        RawRwLockUpgradeTimed!();
        RwLock!();
        RwLockUpgradableReadGuard!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < R : RawRwLockUpgradeTimed , T : ? Sized > RwLock < R , T > { # [doc = " Attempts to acquire this `RwLock` with upgradable read access until a timeout"] # [doc = " is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " `None` is returned. Otherwise, an RAII guard is returned which will"] # [doc = " release the shared access when it is dropped."] # [inline] # [track_caller] pub fn try_upgradable_read_for (& self , timeout : R :: Duration ,) -> Option < RwLockUpgradableReadGuard < '_ , R , T > > { if self . raw . try_lock_upgradable_for (timeout) { Some (unsafe { self . make_upgradable_guard_unchecked () }) } else { None } } # [doc = " Attempts to acquire this `RwLock` with upgradable read access until a timeout"] # [doc = " is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " `None` is returned. Otherwise, an RAII guard is returned which will"] # [doc = " release the shared access when it is dropped."] # [inline] # [track_caller] pub fn try_upgradable_read_until (& self , timeout : R :: Instant ,) -> Option < RwLockUpgradableReadGuard < '_ , R , T > > { if self . raw . try_lock_upgradable_until (timeout) { Some (unsafe { self . make_upgradable_guard_unchecked () }) } else { None } } # [doc = " Attempts to lock this `RwLock` with upgradable access until a timeout is reached, through an `Arc`."] # [doc = ""] # [doc = " This method is similar to the `try_upgradable_read_for` method; however, it requires the `RwLock` to be"] # [doc = " inside of an `Arc` and the resulting read guard has no lifetime requirements."] # [cfg (feature = "arc_lock")] # [inline] # [track_caller] pub fn try_upgradable_read_arc_for (self : & Arc < Self > , timeout : R :: Duration ,) -> Option < ArcRwLockUpgradableReadGuard < R , T > > { if self . raw . try_lock_upgradable_for (timeout) { Some (unsafe { self . make_upgradable_arc_guard_unchecked () }) } else { None } } # [doc = " Attempts to lock this `RwLock` with upgradable access until a timeout is reached, through an `Arc`."] # [doc = ""] # [doc = " This method is similar to the `try_upgradable_read_until` method; however, it requires the `RwLock` to be"] # [doc = " inside of an `Arc` and the resulting read guard has no lifetime requirements."] # [cfg (feature = "arc_lock")] # [inline] # [track_caller] pub fn try_upgradable_read_arc_until (self : & Arc < Self > , timeout : R :: Instant ,) -> Option < ArcRwLockUpgradableReadGuard < R , T > > { if self . raw . try_lock_upgradable_until (timeout) { Some (unsafe { self . make_upgradable_arc_guard_unchecked () }) } else { None } } }
    };
}

impl_118!();