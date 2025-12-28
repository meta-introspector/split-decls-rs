macro_rules! deps {
    () => {
        ArrayLike!();
        Result!();
        ArrayVec!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < A : ArrayLike > fmt :: Debug for ArrayVec < A > where A :: Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_141!();