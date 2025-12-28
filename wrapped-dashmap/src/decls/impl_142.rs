macro_rules! deps {
    () => {
        RawRwLock!();
        RwLockReadGuardDetached!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < R : RawRwLock > Drop for RwLockReadGuardDetached < '_ , R > { fn drop (& mut self) { unsafe { self . lock . unlock_shared () ; } } }
    };
}

impl_142!();