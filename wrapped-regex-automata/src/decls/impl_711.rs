macro_rules! deps {
    () => {
        PoolGuard!();
    };
}

macro_rules! impl_711 {
    () => {
        deps!();
        impl < 'a , T : Send , F : Fn () -> T > core :: ops :: DerefMut for PoolGuard < 'a , T , F > { # [inline] fn deref_mut (& mut self) -> & mut T { self . 0 . value_mut () } }
    };
}

impl_711!();