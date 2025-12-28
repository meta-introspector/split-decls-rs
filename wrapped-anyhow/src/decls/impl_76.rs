macro_rules! deps {
    () => {
        ErrorImpl!();
        Result!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < E > Debug for ErrorImpl < E > where E : Debug , { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { unsafe { ErrorImpl :: debug (self . erase () , formatter) } } }
    };
}

impl_76!();