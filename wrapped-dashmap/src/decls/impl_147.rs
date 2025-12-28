macro_rules! deps {
    () => {
        RwLockReadGuardDetached!();
        RwLockWriteGuardDetached!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockDowngrade > RwLockWriteGuardDetached < 'a , R > { # [doc = " # Safety"] # [doc = ""] # [doc = " The associated data must not mut mutated after downgrading"] pub (crate) unsafe fn downgrade (self) -> RwLockReadGuardDetached < 'a , R > { let this = ManuallyDrop :: new (self) ; unsafe { this . lock . downgrade () } RwLockReadGuardDetached { lock : this . lock , _marker : this . _marker , } } }
    };
}

impl_147!();