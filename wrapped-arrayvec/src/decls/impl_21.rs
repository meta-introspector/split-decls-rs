macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < const CAP : usize > fmt :: Display for ArrayString < CAP > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_21!();