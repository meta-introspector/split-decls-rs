macro_rules! deps {
    () => {
        Result!();
        Formatter!();
        Number!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        impl Debug for Number { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "Number({})" , self) } }
    };
}

impl_542!();