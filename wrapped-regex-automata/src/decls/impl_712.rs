macro_rules! deps {
    () => {
        PoolGuard!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        impl < 'a , T : Send + core :: fmt :: Debug , F : Fn () -> T > core :: fmt :: Debug for PoolGuard < 'a , T , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple ("PoolGuard") . field (& self . 0) . finish () } }
    };
}

impl_712!()