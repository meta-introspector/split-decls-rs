macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T : fmt :: Display + ? Sized , A : Allocator > fmt :: Display for Box < T , A > { # [inline (always)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& * * self , f) } }
    };
}

impl_49!();