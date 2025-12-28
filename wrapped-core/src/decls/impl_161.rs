macro_rules! deps {
    () => {
        Interface!();
        InterfaceRef!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < I : Interface > core :: ops :: Deref for InterfaceRef < '_ , I > { type Target = I ; # [inline (always)] fn deref (& self) -> & I { unsafe { core :: mem :: transmute (self) } } }
    };
}

impl_161!()