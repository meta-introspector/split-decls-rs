macro_rules! deps {
    () => {
        ConstArrayLength!();
        IntoArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T , const N : usize > From < [T ; N] > for GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn from (value : [T ; N]) -> Self { GenericArray :: from_array (value) } }
    };
}

impl_37!()