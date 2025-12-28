macro_rules! deps {
    () => {
        PoolGuard!();
    };
}

macro_rules! impl_710 {
    () => {
        deps!();
        impl < 'a , T : Send , F : Fn () -> T > core :: ops :: Deref for PoolGuard < 'a , T , F > { type Target = T ; # [inline] fn deref (& self) -> & T { self . 0 . value () } }
    };
}

impl_710!();