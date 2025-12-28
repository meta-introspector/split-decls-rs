macro_rules! deps {
    () => {
        Flatten!();
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        unsafe impl < 'a , T , N , M > Flatten < T , N , M > for & 'a mut GenericArray < GenericArray < T , N > , M > where N : ArrayLength + Mul < M > , M : ArrayLength , Prod < N , M > : ArrayLength , { type Output = & 'a mut GenericArray < T , Prod < N , M > > ; # [inline (always)] fn flatten (self) -> Self :: Output { unsafe { mem :: transmute (self) } } }
    };
}

impl_137!();