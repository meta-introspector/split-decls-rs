macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
        Unflatten!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        unsafe impl < T , NM , N > Unflatten < T , NM , N > for GenericArray < T , NM > where NM : ArrayLength + Div < N > , N : ArrayLength , Quot < NM , N > : ArrayLength , { type Output = GenericArray < GenericArray < T , N > , Quot < NM , N > > ; # [inline (always)] fn unflatten (self) -> Self :: Output { unsafe { crate :: const_transmute (self) } } }
    };
}

impl_138!();