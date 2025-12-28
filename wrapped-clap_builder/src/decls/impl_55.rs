macro_rules! deps {
    () => {
        Arg!();
        Styles!();
        Result!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Display for Arg { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { let plain = Styles :: plain () ; self . stylized (& plain , None) . fmt (f) } }
    };
}

impl_55!()