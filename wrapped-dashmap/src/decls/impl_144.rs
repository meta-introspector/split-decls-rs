macro_rules! deps {
    () => {
        RawRwLock!();
        RwLockWriteGuardDetached!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < R : RawRwLock > Drop for RwLockWriteGuardDetached < '_ , R > { fn drop (& mut self) { unsafe { self . lock . unlock_exclusive () ; } } }
    };
}

impl_144!()