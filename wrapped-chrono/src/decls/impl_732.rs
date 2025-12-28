macro_rules! deps {
    () => {
        ParseMonthError!();
    };
}

macro_rules! impl_732 {
    () => {
        deps!();
        impl fmt :: Display for ParseMonthError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ParseMonthError {{ .. }}") } }
    };
}

impl_732!();