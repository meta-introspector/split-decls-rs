macro_rules! deps {
    () => {
        ConstArrayLength!();
        IntoArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'a , T , const N : usize > From < & 'a mut [T ; N] > for & 'a mut GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn from (slice : & 'a mut [T ; N]) -> Self { unsafe { & mut * (slice . as_mut_ptr () as * mut GenericArray < T , ConstArrayLength < N > >) } } }
    };
}

impl_40!();