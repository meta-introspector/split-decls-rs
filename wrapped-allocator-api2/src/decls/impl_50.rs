macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T : fmt :: Debug + ? Sized , A : Allocator > fmt :: Debug for Box < T , A > { # [inline (always)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_50!()