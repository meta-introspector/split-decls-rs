macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T : ? Sized + Ord , A : Allocator > Ord for Box < T , A > { # [inline (always)] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
    };
}

impl_34!();