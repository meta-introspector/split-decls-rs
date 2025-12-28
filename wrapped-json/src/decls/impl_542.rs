macro_rules! deps {
    () => {
        Formatter!();
        Number!();
        Result!();
    };
}

macro_rules! impl_542 {
    () => {
        deps!();
        impl Debug for Number { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "Number({})" , self) } }
    };
}

impl_542!()