macro_rules! deps {
    () => {
        StyleDisplay!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl core :: fmt :: Display for StyleDisplay { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . 0 . fmt_to (f) } }
    };
}

impl_61!();