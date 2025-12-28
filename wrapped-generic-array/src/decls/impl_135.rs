macro_rules! deps {
    () => {
        Flatten!();
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        unsafe impl < T , N , M > Flatten < T , N , M > for GenericArray < GenericArray < T , N > , M > where N : ArrayLength + Mul < M > , M : ArrayLength , Prod < N , M > : ArrayLength , { type Output = GenericArray < T , Prod < N , M > > ; # [inline (always)] fn flatten (self) -> Self :: Output { unsafe { crate :: const_transmute (self) } } }
    };
}

impl_135!()