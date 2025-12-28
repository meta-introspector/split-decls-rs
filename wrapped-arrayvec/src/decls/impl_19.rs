macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < const CAP : usize > fmt :: Debug for ArrayString < CAP > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_19!()