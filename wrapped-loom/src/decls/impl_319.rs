macro_rules! deps {
    () => {
        RwLockWriteGuard!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < 'a , T > ops :: DerefMut for RwLockWriteGuard < 'a , T > { fn deref_mut (& mut self) -> & mut T { self . data . as_mut () . unwrap () . deref_mut () } }
    };
}

impl_319!()