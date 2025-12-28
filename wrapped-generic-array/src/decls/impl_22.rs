macro_rules! deps {
    () => {
        GenericArrayImplOdd!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T : Clone , U : Clone > Clone for GenericArrayImplOdd < T , U > { # [inline (always)] fn clone (& self) -> GenericArrayImplOdd < T , U > { unsafe { core :: hint :: unreachable_unchecked () } } }
    };
}

impl_22!()