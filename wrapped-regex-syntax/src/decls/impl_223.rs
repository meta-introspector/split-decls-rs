macro_rules! deps {
    () => {
        Hir!();
        Result!();
        Formatter!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Hir { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . kind . fmt (f) } }
    };
}

impl_223!()