macro_rules! deps {
    () => {
        RwLock!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < T > From < T > for RwLock < T > { # [doc = " Creates a new rwlock in an unlocked state ready for use."] # [doc = " This is equivalent to [`RwLock::new`]."] fn from (t : T) -> Self { Self :: new (t) } }
    };
}

impl_315!()