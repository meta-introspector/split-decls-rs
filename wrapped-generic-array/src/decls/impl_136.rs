macro_rules! deps {
    () => {
        ArrayLength!();
        Flatten!();
        GenericArray!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        unsafe impl < 'a , T , N , M > Flatten < T , N , M > for & 'a GenericArray < GenericArray < T , N > , M > where N : ArrayLength + Mul < M > , M : ArrayLength , Prod < N , M > : ArrayLength , { type Output = & 'a GenericArray < T , Prod < N , M > > ; # [inline (always)] fn flatten (self) -> Self :: Output { unsafe { mem :: transmute (self) } } }
    };
}

impl_136!();