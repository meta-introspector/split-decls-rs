macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Vec < T , A > { # [inline (always)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_177!();