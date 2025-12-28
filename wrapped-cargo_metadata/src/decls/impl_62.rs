macro_rules! deps {
    () => {
        Source!();
        Result!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl fmt :: Display for Source { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . repr , f) } }
    };
}

impl_62!();