macro_rules! deps {
    () => {
        PropertyNamesShort!();
        NamedEnumeratedProperty!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > core :: fmt :: Debug for PropertyNamesShort < T > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("PropertyNamesShort") . finish () } }
    };
}

impl_46!();