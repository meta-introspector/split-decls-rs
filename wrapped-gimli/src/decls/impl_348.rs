macro_rules! deps {
    () => {
        Attributes!();
        Result!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl Debug for Attributes { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_348!()