macro_rules! deps {
    () => {
        AlwaysReady!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < T , F : Fn () -> T > core :: fmt :: Debug for AlwaysReady < T , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("AlwaysReady") . finish () } }
    };
}

impl_201!();