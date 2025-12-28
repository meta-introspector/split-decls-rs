macro_rules! deps {
    () => {
        Result!();
        ErrorImpl!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < E > Display for ErrorImpl < E > where E : Display , { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { unsafe { Display :: fmt (ErrorImpl :: error (self . erase ()) , formatter) } } }
    };
}

impl_77!();