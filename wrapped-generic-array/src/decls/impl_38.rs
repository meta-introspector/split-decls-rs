macro_rules! deps {
    () => {
        ConstArrayLength!();
        IntoArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T , const N : usize > From < GenericArray < T , ConstArrayLength < N > > > for [T ; N] where Const < N > : IntoArrayLength , { # [inline (always)] fn from (value : GenericArray < T , ConstArrayLength < N > >) -> Self { value . into_array () } }
    };
}

impl_38!();