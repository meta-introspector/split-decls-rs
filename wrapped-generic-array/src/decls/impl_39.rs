macro_rules! deps {
    () => {
        ConstArrayLength!();
        GenericArray!();
        IntoArrayLength!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a , T , const N : usize > From < & 'a [T ; N] > for & 'a GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn from (slice : & 'a [T ; N]) -> Self { unsafe { & * (slice . as_ptr () as * const GenericArray < T , ConstArrayLength < N > >) } } }
    };
}

impl_39!();