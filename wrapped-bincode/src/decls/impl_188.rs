macro_rules! deps {
    () => {
        Compat!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < T > core :: fmt :: Debug for Compat < T > where T : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("Compat") . field (& self . 0) . finish () } }
    };
}

impl_188!()