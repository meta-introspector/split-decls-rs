macro_rules! deps {
    () => {
        ParseMonthError!();
    };
}

macro_rules! impl_733 {
    () => {
        deps!();
        impl fmt :: Debug for ParseMonthError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ParseMonthError {{ .. }}") } }
    };
}

impl_733!()