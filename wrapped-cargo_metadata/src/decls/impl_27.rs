macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl fmt :: Display for Source { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . repr , f) } }
    };
}

impl_27!()