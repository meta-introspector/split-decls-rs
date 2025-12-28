macro_rules! deps {
    () => {
        IntoArrayLength!();
        GenericArray!();
        ConstArrayLength!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < T , const N : usize > AsRef < [T ; N] > for GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn as_ref (& self) -> & [T ; N] { unsafe { core :: mem :: transmute (self) } } }
    };
}

impl_41!()