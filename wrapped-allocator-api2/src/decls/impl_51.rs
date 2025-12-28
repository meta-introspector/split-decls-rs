macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > fmt :: Pointer for Box < T , A > { # [inline (always)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ptr : * const T = & * * self ; fmt :: Pointer :: fmt (& ptr , f) } }
    };
}

impl_51!()