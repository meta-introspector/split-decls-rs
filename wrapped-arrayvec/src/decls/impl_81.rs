macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T , const CAP : usize > fmt :: Debug for ArrayVec < T , CAP > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { (* * self) . fmt (f) } }
    };
}

impl_81!();