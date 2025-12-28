macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesLong!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > core :: fmt :: Debug for PropertyNamesLong < T > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("PropertyNamesLong") . finish () } }
    };
}

impl_37!();