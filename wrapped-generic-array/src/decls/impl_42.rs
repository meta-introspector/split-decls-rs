macro_rules! deps {
    () => {
        IntoArrayLength!();
        GenericArray!();
        ConstArrayLength!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T , const N : usize > AsMut < [T ; N] > for GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn as_mut (& mut self) -> & mut [T ; N] { unsafe { core :: mem :: transmute (self) } } }
    };
}

impl_42!();