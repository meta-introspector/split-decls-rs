macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! impl_707 {
    () => {
        deps!();
        impl < T : core :: fmt :: Debug , F > core :: fmt :: Debug for Pool < T , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple ("Pool") . field (& self . 0) . finish () } }
    };
}

impl_707!();