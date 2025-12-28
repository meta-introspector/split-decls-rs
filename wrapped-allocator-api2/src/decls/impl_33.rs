macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T : ? Sized + PartialOrd , A : Allocator > PartialOrd for Box < T , A > { # [inline (always)] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } # [inline (always)] fn lt (& self , other : & Self) -> bool { PartialOrd :: lt (& * * self , & * * other) } # [inline (always)] fn le (& self , other : & Self) -> bool { PartialOrd :: le (& * * self , & * * other) } # [inline (always)] fn ge (& self , other : & Self) -> bool { PartialOrd :: ge (& * * self , & * * other) } # [inline (always)] fn gt (& self , other : & Self) -> bool { PartialOrd :: gt (& * * self , & * * other) } }
    };
}

impl_33!();