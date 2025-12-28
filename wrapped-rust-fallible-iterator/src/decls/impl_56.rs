macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < I : core :: fmt :: Debug , F > core :: fmt :: Debug for Map < I , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("Map") . field ("iter" , & self . it) . finish () } }
    };
}

impl_56!()