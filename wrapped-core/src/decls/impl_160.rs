macro_rules! deps {
    () => {
        InterfaceRef!();
        Interface!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'a , 'i : 'a , I : Interface > From < & 'i I > for InterfaceRef < 'a , I > { # [inline (always)] fn from (interface : & 'a I) -> Self { InterfaceRef :: from_interface (interface) } }
    };
}

impl_160!();