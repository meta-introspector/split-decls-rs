macro_rules! deps {
    () => {
        InterfaceRef!();
        Interface!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < I : core :: fmt :: Debug + Interface > core :: fmt :: Debug for InterfaceRef < '_ , I > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { < I as core :: fmt :: Debug > :: fmt (& * * self , f) } }
    };
}

impl_158!()