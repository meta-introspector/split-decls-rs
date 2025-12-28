macro_rules! deps {
    () => {
        ParseWeekdayError!();
    };
}

macro_rules! impl_705 {
    () => {
        deps!();
        impl fmt :: Debug for ParseWeekdayError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ParseWeekdayError {{ .. }}") } }
    };
}

impl_705!()