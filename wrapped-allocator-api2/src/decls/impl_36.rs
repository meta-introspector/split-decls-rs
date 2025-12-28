macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T : ? Sized + Hash , A : Allocator > Hash for Box < T , A > { # [inline (always)] fn hash < H : Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
    };
}

impl_36!()