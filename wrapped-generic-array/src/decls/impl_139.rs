macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
        Unflatten!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        unsafe impl < 'a , T , NM , N > Unflatten < T , NM , N > for & 'a GenericArray < T , NM > where NM : ArrayLength + Div < N > , N : ArrayLength , Quot < NM , N > : ArrayLength , { type Output = & 'a GenericArray < GenericArray < T , N > , Quot < NM , N > > ; # [inline (always)] fn unflatten (self) -> Self :: Output { unsafe { mem :: transmute (self) } } }
    };
}

impl_139!();