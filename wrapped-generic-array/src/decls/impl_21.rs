macro_rules! deps {
    () => {
        GenericArrayImplEven!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T : Clone , U : Clone > Clone for GenericArrayImplEven < T , U > { # [inline (always)] fn clone (& self) -> GenericArrayImplEven < T , U > { unsafe { core :: hint :: unreachable_unchecked () } } }
    };
}

impl_21!()